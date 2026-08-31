// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crate::keytool::CommandOutput;
use crate::keytool::read_authority_keypair_from_file;
use crate::keytool::read_keypair_from_file;

use super::KeyToolCommand;
use super::write_keypair_to_file;
use anyhow::Ok;
use fastcrypto::ed25519::Ed25519KeyPair;
use fastcrypto::encoding::Base64;
use fastcrypto::encoding::Encoding;
use fastcrypto::encoding::Hex;
use fastcrypto::traits::ToFromBytes;
use haneul_keys::key_identity::KeyIdentity;
use haneul_keys::keystore::{AccountKeystore, FileBasedKeystore, InMemKeystore, Keystore};
use haneul_sdk::wallet_context::WalletContext;
use haneul_types::base_types::HaneulAddress;
use haneul_types::base_types::ObjectDigest;
use haneul_types::base_types::ObjectID;
use haneul_types::base_types::SequenceNumber;
use haneul_types::crypto::AuthorityKeyPair;
use haneul_types::crypto::Ed25519HaneulSignature;
use haneul_types::crypto::EncodeDecodeBase64;
use haneul_types::crypto::HaneulKeyPair;
use haneul_types::crypto::HaneulSignatureInner;
use haneul_types::crypto::Secp256k1HaneulSignature;
use haneul_types::crypto::Secp256r1HaneulSignature;
use haneul_types::crypto::Signature;
use haneul_types::crypto::SignatureScheme;
use haneul_types::crypto::get_key_pair;
use haneul_types::crypto::get_key_pair_from_rng;
use haneul_types::transaction::TEST_ONLY_GAS_UNIT_FOR_TRANSFER;
use haneul_types::transaction::TransactionData;
use rand::SeedableRng;
use rand::rngs::StdRng;
use shared_crypto::intent::Intent;
use shared_crypto::intent::IntentScope;
use tempfile::TempDir;
use tokio::test;

const TEST_MNEMONIC: &str = "result crisp session latin must fruit genuine question prevent start coconut brave speak student dismiss";

#[test]
async fn test_addresses_command() -> Result<(), anyhow::Error> {
    // Add 3 Ed25519 KeyPairs as default
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(3));

    // Add another 3 Secp256k1 KeyPairs
    for _ in 0..3 {
        keystore
            .import(None, HaneulKeyPair::Secp256k1(get_key_pair().1))
            .await?;
    }

    let mut context = WalletContext::new_for_tests(keystore, None, None);

    // List all addresses with flag
    KeyToolCommand::List {
        sort_by_alias: true,
    }
    .execute(&mut context)
    .await
    .unwrap();
    Ok(())
}

#[test]
async fn test_flag_in_signature_and_keypair() -> Result<(), anyhow::Error> {
    let mut keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));

    keystore
        .import(None, HaneulKeyPair::Secp256k1(get_key_pair().1))
        .await?;
    keystore
        .import(None, HaneulKeyPair::Ed25519(get_key_pair().1))
        .await?;

    for pk in keystore.entries() {
        let pk1 = pk.clone();
        let sig = keystore
            .sign_secure(&(&pk).into(), b"hello", Intent::haneul_transaction())
            .await?;
        match sig {
            Signature::Ed25519HaneulSignature(_) => {
                // signature contains corresponding flag
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Ed25519HaneulSignature::SCHEME.flag()
                );
                // keystore stores pubkey with corresponding flag
                assert!(pk1.flag() == Ed25519HaneulSignature::SCHEME.flag())
            }
            Signature::Secp256k1HaneulSignature(_) => {
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Secp256k1HaneulSignature::SCHEME.flag()
                );
                assert!(pk1.flag() == Secp256k1HaneulSignature::SCHEME.flag())
            }
            Signature::Secp256r1HaneulSignature(_) => {
                assert_eq!(
                    *sig.as_ref().first().unwrap(),
                    Secp256r1HaneulSignature::SCHEME.flag()
                );
                assert!(pk1.flag() == Secp256r1HaneulSignature::SCHEME.flag())
            }
        }
    }
    Ok(())
}

#[test]
async fn test_read_write_keystore_with_flag() {
    let dir = tempfile::TempDir::new().unwrap();

    // create Secp256k1 keypair
    let kp_secp = HaneulKeyPair::Secp256k1(get_key_pair().1);
    let addr_secp: HaneulAddress = (&kp_secp.public()).into();
    let fp_secp = dir.path().join(format!("{}.key", addr_secp));
    let fp_secp_2 = fp_secp.clone();

    // write Secp256k1 keypair to file
    let res = write_keypair_to_file(&kp_secp, &fp_secp);
    assert!(res.is_ok());

    // read from file as enum KeyPair success
    let kp_secp_read = read_keypair_from_file(fp_secp);
    assert!(kp_secp_read.is_ok());

    // KeyPair wrote into file is the same as read
    assert_eq!(
        kp_secp_read.unwrap().public().as_ref(),
        kp_secp.public().as_ref()
    );

    // read as AuthorityKeyPair fails
    let kp_secp_read = read_authority_keypair_from_file(fp_secp_2);
    assert!(kp_secp_read.is_err());

    // create Ed25519 keypair
    let kp_ed = HaneulKeyPair::Ed25519(get_key_pair().1);
    let addr_ed: HaneulAddress = (&kp_ed.public()).into();
    let fp_ed = dir.path().join(format!("{}.key", addr_ed));
    let fp_ed_2 = fp_ed.clone();

    // write Ed25519 keypair to file
    let res = write_keypair_to_file(&kp_ed, &fp_ed);
    assert!(res.is_ok());

    // read from file as enum KeyPair success
    let kp_ed_read = read_keypair_from_file(fp_ed);
    assert!(kp_ed_read.is_ok());

    // KeyPair wrote into file is the same as read
    assert_eq!(
        kp_ed_read.unwrap().public().as_ref(),
        kp_ed.public().as_ref()
    );

    // read from file as AuthorityKeyPair success
    let kp_ed_read = read_authority_keypair_from_file(fp_ed_2);
    assert!(kp_ed_read.is_err());
}

#[test]
async fn test_haneul_operations_config() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("haneul.keystore");
    let path1 = path.clone();
    // This is the hardcoded keystore in haneul-operation: https://github.com/GeunhwaJeong/haneul-operations/blob/af04c9d3b61610dbb36401aff6bef29d06ef89f8/docker/config/generate/static/haneul.keystore
    // If this test fails, address hardcoded in haneul-operations is likely needed be updated.
    let kp = HaneulKeyPair::decode_base64("ANRj4Rx5FZRehqwrctiLgZDPrY/3tI5+uJLCdaXPCj6C").unwrap();
    let contents = vec![kp.encode_base64()];
    let res = std::fs::write(path, serde_json::to_string_pretty(&contents).unwrap());
    assert!(res.is_ok());
    let read = FileBasedKeystore::load_or_create(&path1);
    assert!(read.is_ok());
    assert_eq!(
        HaneulAddress::from_str("7d20dcdb2bca4f508ea9613994683eb4e76e9c4ed371169677c1be02aaf0b58e")
            .unwrap(),
        read.unwrap().addresses()[0]
    );

    // This is the hardcoded keystore in haneul-operation: https://github.com/GeunhwaJeong/haneul-operations/blob/af04c9d3b61610dbb36401aff6bef29d06ef89f8/docker/config/generate/static/haneul-benchmark.keystore
    // If this test fails, address hardcoded in haneul-operations is likely needed be updated.
    let path2 = temp_dir.path().join("haneul-benchmark.keystore");
    let path3 = path2.clone();
    let kp = HaneulKeyPair::decode_base64("APCWxPNCbgGxOYKeMfPqPmXmwdNVyau9y4IsyBcmC14A").unwrap();
    let contents = vec![kp.encode_base64()];
    let res = std::fs::write(path2, serde_json::to_string_pretty(&contents).unwrap());
    assert!(res.is_ok());
    let read = FileBasedKeystore::load_or_create(&path3);
    assert_eq!(
        HaneulAddress::from_str("160ef6ce4f395208a12119c5011bf8d8ceb760e3159307c819bd0197d154d384")
            .unwrap(),
        read.unwrap().addresses()[0]
    );
}

#[test]
async fn test_load_keystore_err() {
    let temp_dir = TempDir::new().unwrap();
    let path = temp_dir.path().join("haneul.keystore");
    let path2 = path.clone();

    // write encoded AuthorityKeyPair without flag byte to file
    let kp: AuthorityKeyPair = get_key_pair_from_rng(&mut StdRng::from_seed([0; 32])).1;
    let contents = kp.encode_base64();
    let res = std::fs::write(path, contents);
    assert!(res.is_ok());

    // cannot load keypair due to missing flag
    assert!(FileBasedKeystore::load_or_create(&path2).is_err());
}

#[test]
async fn test_private_keys_import_export() -> Result<(), anyhow::Error> {
    // private key in Bech32, private key in Hex, private key in Base64, derived Haneul address in Hex
    const TEST_CASES: &[(&str, &str, &str, &str)] = &[
        (
            "haneulprivkey1qzwant3kaegmjy4qxex93s0jzvemekkjmyv3r2sjwgnv2y479pgsyzqxlrx",
            "0x9dd9ae36ee51b912a0364c58c1f21333bcdad2d91911aa127226c512be285102",
            "AJ3ZrjbuUbkSoDZMWMHyEzO82tLZGRGqEnImxRK+KFEC",
            "0x90f3e6d73b5730f16974f4df1d3441394ebae62186baf83608599f226455afa7",
        ),
        (
            "haneulprivkey1qrh2sjl88rze74hwjndw3l26dqyz63tea5u9frtwcsqhmfk9vxdlxt0t23h",
            "0xeea84be738c59f56ee94dae8fd5a68082d4579ed38548d6ec4017da6c5619bf3",
            "AO6oS+c4xZ9W7pTa6P1aaAgtRXntOFSNbsQBfabFYZvz",
            "0xfd233cd9a5dd7e577f16fa523427c75fbc382af1583c39fdf1c6747d2ed807a3",
        ),
        (
            "haneulprivkey1qzg73qyvfz0wpnyectkl08nrhe4pgnu0vqx8gydu96qx7uj4wyr8g55cefw",
            "0x91e8808c489ee0cc99c2edf79e63be6a144f8f600c7411bc2e806f7255710674",
            "AJHogIxInuDMmcLt955jvmoUT49gDHQRvC6Ab3JVcQZ0",
            "0x81aaefa4a883e72e8b6ccd3bec307e25fe3d79b14e43b778695c55dcec42f4f0",
        ),
    ];
    // assert correctness
    for (private_key, private_key_hex, private_key_base64, address) in TEST_CASES {
        let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        let mut context = WalletContext::new_for_tests(keystore, None, None);
        KeyToolCommand::Import {
            alias: None,
            input_string: private_key.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut context)
        .await?;
        let kp = HaneulKeyPair::decode(private_key).unwrap();
        let kp_from_hex = HaneulKeyPair::Ed25519(
            Ed25519KeyPair::from_bytes(&Hex::decode(private_key_hex).unwrap()).unwrap(),
        );
        assert_eq!(kp, kp_from_hex);

        let kp_from_base64 = HaneulKeyPair::decode_base64(private_key_base64).unwrap();
        assert_eq!(kp, kp_from_base64);

        let addr = HaneulAddress::from_str(address).unwrap();
        assert_eq!(HaneulAddress::from(&kp.public()), addr);
        assert!(context.config.keystore.addresses().contains(&addr));

        // Export output shows the private key in Bech32
        let output = KeyToolCommand::Export {
            key_identity: KeyIdentity::Address(addr),
        }
        .execute(&mut context)
        .await?;
        match output {
            CommandOutput::Export(exported) => {
                assert_eq!(exported.exported_private_key, private_key.to_string());
            }
            _ => panic!("unexpected output"),
        }
    }

    for (private_key, _, _, addr) in TEST_CASES {
        let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        let mut context = WalletContext::new_for_tests(keystore, None, None);
        // assert failure when private key is malformed
        let output = KeyToolCommand::Import {
            alias: None,
            input_string: private_key[1..].to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut context)
        .await;
        assert!(output.is_err());

        // importing an hex encoded string should fail
        let output = KeyToolCommand::Import {
            alias: None,
            input_string: addr.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut context)
        .await;
        assert!(output.is_err());
    }

    Ok(())
}

#[test]
async fn test_mnemonics_ed25519() -> Result<(), anyhow::Error> {
    // Test case matches with /haneullabs/haneul/sdk/typescript/test/unit/cryptography/ed25519-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [
        [
            "film crazy soon outside stand loop subway crumble thrive popular green nuclear struggle pistol arm wife phrase warfare march wheat nephew ask sunny firm",
            "haneulprivkey1qq3denfafqukqq787x2lm3w0xz6l7ervh972qd5qpg6jpatgj0nlqpkkfd5",
            "055c03c8c3403919d4e09932dcdda72a2fc82bfe3271f997757eb3b4da7bb1c6",
        ],
        [
            "require decline left thought grid priority false tiny gasp angle royal system attack beef setup reward aunt skill wasp tray vital bounce inflict level",
            "haneulprivkey1qzr34yn7lwexhmqdk8tpmqc9kym66spt7emeu0jkqsfa5hk0w5kgxwsw34p",
            "6f86f3c584f9a645691bf1003eb6a1e4fdb6dda4c61548d2f6ebd4994233bc62",
        ],
        [
            "organ crash swim stick traffic remember army arctic mesh slice swear summer police vast chaos cradle squirrel hood useless evidence pet hub soap lake",
            "haneulprivkey1qqljwx0h9wq07sujftzygpacxwmkdws08apggv5068vrfpahspzzstnanhk",
            "18ee8052cb5152c86a472be66075fefd44f7d1bb041b48a5a08b9b65e9f1612a",
        ],
    ];

    for t in TEST_CASES {
        let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        let mut context = WalletContext::new_for_tests(keystore, None, None);
        KeyToolCommand::Import {
            alias: None,
            input_string: t[0].to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: None,
        }
        .execute(&mut context)
        .await?;
        let kp = HaneulKeyPair::decode(t[1]).unwrap();
        let addr = HaneulAddress::from_str(t[2]).unwrap();
        assert_eq!(HaneulAddress::from(&kp.public()), addr);
        assert!(context.config.keystore.addresses().contains(&addr));
    }
    Ok(())
}

#[test]
async fn test_mnemonics_secp256k1() -> Result<(), anyhow::Error> {
    // Test case matches with /haneullabs/haneul/sdk/typescript/test/unit/cryptography/secp256k1-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [
        [
            "film crazy soon outside stand loop subway crumble thrive popular green nuclear struggle pistol arm wife phrase warfare march wheat nephew ask sunny firm",
            "haneulprivkey1qyxagt2a78lc0s3328xeq0nltyqpyda0vq0t4taq5dmpv645a4wnkvnxy9u",
            "3324c73fa34509db8d0baaf389a412449509629bdb06d826fbfb50082e6f8b6e",
        ],
        [
            "require decline left thought grid priority false tiny gasp angle royal system attack beef setup reward aunt skill wasp tray vital bounce inflict level",
            "haneulprivkey1qxt9wll7mf06nl23ftvewtm9stmvrxvhuzdgq4fph9xljat0zsgm6ccaanc",
            "eb8e733513c797118e0cbca10eafa77fb31c9fe8f74eeb931cf351280a642617",
        ],
        [
            "organ crash swim stick traffic remember army arctic mesh slice swear summer police vast chaos cradle squirrel hood useless evidence pet hub soap lake",
            "haneulprivkey1q9l4ucn5hj2w2mj3v4f5h6qng070j2drd5wgt22frk0jq3ar828sgdlwkjq",
            "6cc36ff91ffc4170d818c81e821988a8360a720b49150702105836cbfa18bd30",
        ],
    ];

    for t in TEST_CASES {
        let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        let mut context = WalletContext::new_for_tests(keystore, None, None);
        KeyToolCommand::Import {
            alias: None,
            input_string: t[0].to_string(),
            key_scheme: SignatureScheme::Secp256k1,
            derivation_path: None,
        }
        .execute(&mut context)
        .await?;
        let kp = HaneulKeyPair::decode(t[1]).unwrap();
        let addr = HaneulAddress::from_str(t[2]).unwrap();
        assert_eq!(HaneulAddress::from(&kp.public()), addr);
        assert!(context.config.keystore.addresses().contains(&addr));
    }
    Ok(())
}

#[test]
async fn test_mnemonics_secp256r1() -> Result<(), anyhow::Error> {
    // Test case matches with /haneullabs/haneul/sdk/typescript/test/unit/cryptography/secp256r1-keypair.test.ts
    const TEST_CASES: [[&str; 3]; 3] = [
        [
            "act wing dilemma glory episode region allow mad tourist humble muffin oblige",
            "haneulprivkey1qfw9yd23qca85dywwp2gc2fkjv766xe44370r8rsttaul77sn5g22vf6fch",
            "0x8dca749c89a17aba44abf789e3b82974e85036102917a572406406164d8ec069",
        ],
        [
            "flag rebel cabbage captain minimum purpose long already valley horn enrich salt",
            "haneulprivkey1q2qgrcj0k5vdhy25fzcgncfl3slc6lvpvq08yspzdtvxj53qxnyqjf5xpte",
            "0x2cc08866814b783f84c29f5aa387c8f63c88a8c2f34a3a7adc7fbf805c687ee7",
        ],
        [
            "area renew bar language pudding trial small host remind supreme cabbage era",
            "haneulprivkey1q285mtvz4z4rjhmrf09s44rg2z5u8jw42utv4x46v7mm8jt5r9fx7uau5en",
            "0x7af553c91a78621cb73fcd5175b6ed43b7b80def8b771759db89c1c3bb14b91d",
        ],
    ];

    for [mnemonics, sk, address] in TEST_CASES {
        let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
        let mut context = WalletContext::new_for_tests(keystore, None, None);
        KeyToolCommand::Import {
            alias: None,
            input_string: mnemonics.to_string(),
            key_scheme: SignatureScheme::Secp256r1,
            derivation_path: None,
        }
        .execute(&mut context)
        .await?;

        let kp = HaneulKeyPair::decode(sk).unwrap();
        let addr = HaneulAddress::from_str(address).unwrap();
        assert_eq!(HaneulAddress::from(&kp.public()), addr);
        assert!(context.config.keystore.addresses().contains(&addr));
    }

    Ok(())
}

#[test]
async fn test_invalid_derivation_path() -> Result<(), anyhow::Error> {
    let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
    let mut context = WalletContext::new_for_tests(keystore, None, None);
    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: Some("m/44'/1'/0'/0/0".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_err()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: Some("m/0'/784'/0'/0/0".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_err()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: Some("m/54'/8282'/0'/0/0".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_err()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::Secp256k1,
            derivation_path: Some("m/54'/8282'/0'/0'/0'".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_err()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::Secp256k1,
            derivation_path: Some("m/44'/8282'/0'/0/0".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_err()
    );

    Ok(())
}

#[test]
async fn test_valid_derivation_path() -> Result<(), anyhow::Error> {
    let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
    let mut context = WalletContext::new_for_tests(keystore, None, None);

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: Some("m/44'/8282'/0'/0'/0'".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_ok()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: Some("m/44'/8282'/0'/0'/1'".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_ok()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::ED25519,
            derivation_path: Some("m/44'/8282'/1'/0'/1'".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_ok()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::Secp256k1,
            derivation_path: Some("m/54'/8282'/0'/0/1".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_ok()
    );

    assert!(
        KeyToolCommand::Import {
            alias: None,
            input_string: TEST_MNEMONIC.to_string(),
            key_scheme: SignatureScheme::Secp256k1,
            derivation_path: Some("m/54'/8282'/1'/0/1".parse().unwrap()),
        }
        .execute(&mut context)
        .await
        .is_ok()
    );
    Ok(())
}

#[test]
async fn test_keytool_bls12381() -> Result<(), anyhow::Error> {
    let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(0));
    let mut context = WalletContext::new_for_tests(keystore, None, None);
    KeyToolCommand::Generate {
        key_scheme: SignatureScheme::BLS12381,
        derivation_path: None,
        word_length: None,
    }
    .execute(&mut context)
    .await?;
    Ok(())
}

#[test]
async fn test_sign_command() -> Result<(), anyhow::Error> {
    // Add a keypair
    let keystore = Keystore::from(InMemKeystore::new_insecure_for_tests(1));
    let mut context = WalletContext::new_for_tests(keystore, None, None);
    let binding = context.config.keystore.addresses();
    let sender = binding.first().unwrap();
    let alias = context.config.keystore.get_alias(sender).unwrap();

    // Create a dummy TransactionData
    let gas = (
        ObjectID::random(),
        SequenceNumber::new(),
        ObjectDigest::random(),
    );
    let gas_price = 1;
    let tx_data = TransactionData::new_pay_haneul(
        *sender,
        vec![gas],
        vec![HaneulAddress::random_for_testing_only()],
        vec![10000],
        gas,
        gas_price * TEST_ONLY_GAS_UNIT_FOR_TRANSFER,
        gas_price,
    )
    .unwrap();

    // Sign an intent message for the transaction data and a passed-in intent with scope as PersonalMessage.
    KeyToolCommand::Sign {
        address: KeyIdentity::Address(*sender),
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: Some(Intent::haneul_app(IntentScope::PersonalMessage)),
    }
    .execute(&mut context)
    .await?;

    // Sign an intent message for the transaction data without intent passed in, so default is used.
    KeyToolCommand::Sign {
        address: KeyIdentity::Address(*sender),
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: None,
    }
    .execute(&mut context)
    .await?;

    // Sign an intent message for the transaction data without intent passed in, so default is used.
    // Use alias for signing instead of the address
    KeyToolCommand::Sign {
        address: KeyIdentity::Alias(alias),
        data: Base64::encode(bcs::to_bytes(&tx_data)?),
        intent: None,
    }
    .execute(&mut context)
    .await?;
    Ok(())
}
