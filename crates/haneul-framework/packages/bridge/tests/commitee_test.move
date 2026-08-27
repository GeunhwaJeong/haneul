// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(deprecated_usage)] // TODO: update tests to not use deprecated governance
module bridge::committee_test;

use bridge::chain_ids;
use bridge::committee::{
    BridgeCommittee,
    CommitteeMember,
    blocklisted,
    bridge_pubkey_bytes,
    create,
    members,
    member_registrations,
    register,
    try_create_next_committee,
    verify_signatures,
    voting_power,
    execute_blocklist,
    make_committee_member,
    make_bridge_committee,
};
use bridge::crypto;
use bridge::message;
use std::unit_test::{destroy, assert_eq};
use haneul::hex;
use haneul::test_scenario;
use haneul::vec_map;
use haneul_system::governance_test_utils::{
    advance_epoch_with_reward_amounts,
    create_haneul_system_state_for_testing,
    create_validator_for_testing,
};
use haneul_system::haneul_system::{Self, HaneulSystemState};

// This is a token transfer message for testing
const TEST_MSG: vector<u8> =
    b"00010a0000000000000000200000000000000000000000000000000000000000000000000000000000000064012000000000000000000000000000000000000000000000000000000000000000c8033930000000000000";

const VALIDATOR1_PUBKEY: vector<u8> =
    b"033e99a541db69bd32040dfe5037fbf5210dafa8151a71e21c5204b05d95ce0a62";
const VALIDATOR2_PUBKEY: vector<u8> =
    b"0286bcc70599ebc420b3b8977ecc60e594bb56749beaa562d7f80a9bdfffcaaa1d";
const VALIDATOR3_PUBKEY: vector<u8> =
    b"033e99a541db69bd32040dfe5037fbf5210dafa8151a71e21c5204b05d95ce0a63";

#[test]
fun test_verify_signatures_good_path() {
    let committee = setup_test();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    // good path
    committee.verify_signatures(
        msg,
        vector[
            hex::decode(
                b"cc185408c86c88f7f74843b4e7dd989bc6b810f523812c1e9d40aead2a62d14c1b3f104d995dc879cdc83b83ca202d43816ade4499ae6c5260e94747f1e4227f00",
            ),
            hex::decode(
                b"fa23dd1320079e5e0d86ad0510472a7c1a7bb7a282c57aade97b8e7a78686f092eade5c9455634a0556d8a189cc7f964492bffb90878f755a6d6bade3683181600",
            ),
        ],
    );

    // Clean up
    destroy(committee)
}

#[test, expected_failure(abort_code = bridge::committee::EDuplicatedSignature)]
fun test_verify_signatures_duplicated_sig() {
    let committee = setup_test();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    // good path
    committee.verify_signatures(
        msg,
        vector[
            hex::decode(
                b"fa23dd1320079e5e0d86ad0510472a7c1a7bb7a282c57aade97b8e7a78686f092eade5c9455634a0556d8a189cc7f964492bffb90878f755a6d6bade3683181600",
            ),
            hex::decode(
                b"fa23dd1320079e5e0d86ad0510472a7c1a7bb7a282c57aade97b8e7a78686f092eade5c9455634a0556d8a189cc7f964492bffb90878f755a6d6bade3683181600",
            ),
        ],
    );
    abort
}

#[test, expected_failure(abort_code = bridge::committee::EInvalidSignature)]
fun test_verify_signatures_invalid_signature() {
    let committee = setup_test();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    // good path
    committee.verify_signatures(
        msg,
        vector[
            hex::decode(
                b"6ffb3e5ce04dd138611c49520fddfbd6778879c2db4696139f53a487043409536c369c6ffaca165ce3886723cfa8b74f3e043e226e206ea25e313ea2215e6caf01",
            ),
        ],
    );
    abort
}

#[test, expected_failure(abort_code = bridge::committee::ESignatureBelowThreshold)]
fun test_verify_signatures_below_threshold() {
    let committee = setup_test();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    // good path
    committee.verify_signatures(
        msg,
        vector[
            hex::decode(
                b"fa23dd1320079e5e0d86ad0510472a7c1a7bb7a282c57aade97b8e7a78686f092eade5c9455634a0556d8a189cc7f964492bffb90878f755a6d6bade3683181600",
            ),
        ],
    );
    abort
}

#[test]
fun test_init_committee() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR1_PUBKEY),
        b"",
        &tx(@0xA, 0),
    );
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR2_PUBKEY),
        b"",
        &tx(@0xC, 0),
    );

    // Check committee before creation
    assert!(committee.members().is_empty());

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = system_state.validator_voting_powers_for_testing();
    committee.try_create_next_committee(voting_powers, 6000, ctx);

    assert_eq!(2, committee.members().length());
    let (_, member0) = committee.members().get_entry_by_idx(0);
    let (_, member1) = committee.members().get_entry_by_idx(1);
    assert_eq!(5000, member0.voting_power());
    assert_eq!(5000, member1.voting_power());

    let members = committee.members();
    assert!(members.length() == 2); // must succeed

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_update_node_url() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[create_validator_for_testing(@0xA, 100, ctx)];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR1_PUBKEY),
        b"test url 1",
        &tx(@0xA, 0),
    );

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = system_state.validator_voting_powers_for_testing();
    committee.try_create_next_committee(voting_powers, 6000, ctx);

    let members = committee.members();
    assert!(members.length() == 1);
    let (_, member) = members.get_entry_by_idx(0);
    assert_eq!(member.http_rest_url(), b"test url 1");

    // Update URL
    committee.update_node_url(
        b"test url 2",
        &tx(@0xA, 0),
    );

    let members = committee.members();
    let (_, member) = members.get_entry_by_idx(0);
    assert_eq!(member.http_rest_url(), b"test url 2");

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test, expected_failure(abort_code = bridge::committee::ESenderIsNotInBridgeCommittee)]
fun test_update_node_url_not_validator() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[create_validator_for_testing(@0xA, 100, ctx)];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration
    committee.register(
        &mut system_state,
        hex::decode(VALIDATOR1_PUBKEY),
        b"test url 1",
        &tx(@0xA, 0),
    );

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = system_state.validator_voting_powers_for_testing();
    committee.try_create_next_committee(voting_powers, 6000, ctx);

    // Update URL should fail for validator @0xB
    committee.update_node_url(
        b"test url",
        &tx(@0xB, 0),
    );

    // test should have failed, abort
    abort
}

#[test, expected_failure(abort_code = bridge::committee::ENotSystemAddress)]
fun test_init_non_system_sender() {
    let mut scenario = test_scenario::begin(@0x1);
    let ctx = test_scenario::ctx(&mut scenario);
    let _committee = create(ctx);

    abort
}

#[test, expected_failure(abort_code = bridge::committee::ESenderNotActiveValidator)]
fun test_init_committee_not_validator() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xD, 0));

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test, expected_failure(abort_code = bridge::committee::EDuplicatePubkey)]
fun test_init_committee_dup_pubkey() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xC, 0));

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_init_committee_validator_become_inactive() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
        create_validator_for_testing(@0xD, 100, ctx),
        create_validator_for_testing(@0xE, 100, ctx),
        create_validator_for_testing(@0xF, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration, 3 validators registered, should have 60% voting power in total
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xC, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR3_PUBKEY), b"", &tx(@0xD, 0));

    // Verify validator registration
    assert_eq!(3, committee.member_registrations().length());

    // Validator 0xA become inactive, total voting power become 50%
    haneul_system::request_remove_validator(&mut system_state, &mut tx(@0xA, 0));
    test_scenario::return_shared(system_state);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // create committee should not create a committe because of not enough stake.
    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = haneul_system::validator_voting_powers_for_testing(&mut system_state);
    try_create_next_committee(&mut committee, voting_powers, 6000, ctx);

    assert!(committee.members().is_empty());

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_update_committee_registration() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));

    // Verify registration info
    assert_eq!(1, committee.member_registrations().length());
    let (address, registration) = committee.member_registrations().get_entry_by_idx(0);
    assert_eq!(@0xA, *address);
    assert!(&hex::decode(VALIDATOR1_PUBKEY) == registration.bridge_pubkey_bytes(), 0);

    // Register again with different pub key.
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xA, 0));

    // Verify registration info, registration count should still be 1
    assert_eq!(1, committee.member_registrations().length());
    let (address, registration) = committee.member_registrations().get_entry_by_idx(0);
    assert_eq!(@0xA, *address);
    assert!(&hex::decode(VALIDATOR2_PUBKEY) == registration.bridge_pubkey_bytes(), 0);

    // teardown
    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test]
fun test_init_committee_not_enough_stake() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);
    test_scenario::next_tx(&mut scenario, @0x0);

    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);

    // validator registration
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));

    // Check committee before creation
    assert!(committee.members().is_empty());

    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = haneul_system::validator_voting_powers_for_testing(&mut system_state);
    try_create_next_committee(&mut committee, voting_powers, 6000, ctx);

    // committee should be empty because registration did not reach min stake threshold.
    assert!(committee.members().is_empty());

    destroy(committee);
    test_scenario::return_shared(system_state);
    test_scenario::end(scenario);
}

#[test, expected_failure(abort_code = bridge::committee::ECommitteeAlreadyInitiated)]
fun test_register_already_initialized() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);

    test_scenario::next_tx(&mut scenario, @0x0);
    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xC, 0));
    assert!(committee.members().is_empty());
    let ctx = test_scenario::ctx(&mut scenario);
    let voting_powers = haneul_system::validator_voting_powers_for_testing(&mut system_state);
    try_create_next_committee(&mut committee, voting_powers, 6000, ctx);

    test_scenario::next_tx(&mut scenario, @0x0);
    assert!(committee.members().length() == 2); // must succeed
    // this fails because committee is already initiated
    committee.register(&mut system_state, hex::decode(VALIDATOR1_PUBKEY), b"", &tx(@0xA, 0));

    abort
}

#[test, expected_failure(abort_code = bridge::committee::EInvalidPubkeyLength)]
fun test_register_bad_pubkey() {
    let mut scenario = test_scenario::begin(@0x0);
    let ctx = test_scenario::ctx(&mut scenario);
    let mut committee = create(ctx);

    let validators = vector[
        create_validator_for_testing(@0xA, 100, ctx),
        create_validator_for_testing(@0xC, 100, ctx),
    ];
    create_haneul_system_state_for_testing(validators, 0, 0, ctx);
    advance_epoch_with_reward_amounts(0, 0, &mut scenario);

    test_scenario::next_tx(&mut scenario, @0x0);
    let mut system_state = test_scenario::take_shared<HaneulSystemState>(&scenario);
    committee.register(&mut system_state, hex::decode(VALIDATOR2_PUBKEY), b"", &tx(@0xC, 0));
    // this fails with invalid public key
    committee.register(&mut system_state, b"029bef8", b"", &tx(@0xA, 0));

    abort
}

fun tx(sender: address, hint: u64): TxContext {
    tx_context::new_from_hint(sender, hint, 1, 0, 0)
}

#[test, expected_failure(abort_code = bridge::committee::ESignatureBelowThreshold)]
fun test_verify_signatures_with_blocked_committee_member() {
    let mut committee = setup_test();
    let msg = message::deserialize_message_test_only(hex::decode(TEST_MSG));
    // good path, this test should have passed in previous test
    committee.verify_signatures(
        msg,
        vector[
            hex::decode(
                b"cc185408c86c88f7f74843b4e7dd989bc6b810f523812c1e9d40aead2a62d14c1b3f104d995dc879cdc83b83ca202d43816ade4499ae6c5260e94747f1e4227f00",
            ),
            hex::decode(
                b"fa23dd1320079e5e0d86ad0510472a7c1a7bb7a282c57aade97b8e7a78686f092eade5c9455634a0556d8a189cc7f964492bffb90878f755a6d6bade3683181600",
            ),
        ],
    );

    let (validator1, member) = committee.members().get_entry_by_idx(0);
    assert!(!member.blocklisted());

    // Block a member
    let blocklist = message::create_blocklist_message(
        chain_ids::haneul_testnet(),
        0,
        0, // type 0 is block
        vector[crypto::ecdsa_pub_key_to_eth_address(validator1)],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    let (_, blocked_member) = committee.members().get_entry_by_idx(0);
    assert!(blocked_member.blocklisted());

    // Verify signature should fail now
    committee.verify_signatures(
        msg,
        vector[
            hex::decode(
                b"cc185408c86c88f7f74843b4e7dd989bc6b810f523812c1e9d40aead2a62d14c1b3f104d995dc879cdc83b83ca202d43816ade4499ae6c5260e94747f1e4227f00",
            ),
            hex::decode(
                b"fa23dd1320079e5e0d86ad0510472a7c1a7bb7a282c57aade97b8e7a78686f092eade5c9455634a0556d8a189cc7f964492bffb90878f755a6d6bade3683181600",
            ),
        ],
    );

    // Clean up
    destroy(committee);
}

#[test, expected_failure(abort_code = bridge::committee::EValidatorBlocklistContainsUnknownKey)]
fun test_execute_blocklist_abort_upon_unknown_validator() {
    let mut committee = setup_test();

    // // val0 and val1 are not blocked yet
    let (validator0, _) = committee.members().get_entry_by_idx(0);
    // assert!(!member0.blocklisted());
    // let (validator1, member1) = committee.members().get_entry_by_idx(1);
    // assert!(!member1.blocklisted());

    let eth_address0 = crypto::ecdsa_pub_key_to_eth_address(validator0);
    let invalid_eth_address1 = x"0000000000000000000000000000000000000000";

    // Blocklist both
    let blocklist = message::create_blocklist_message(
        chain_ids::haneul_testnet(),
        0, // seq
        0, // type 0 is blocklist
        vector[eth_address0, invalid_eth_address1],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // Clean up
    destroy(committee);
}

#[test]
fun test_execute_blocklist() {
    let mut committee = setup_test();

    // val0 and val1 are not blocked yet
    let (validator0, member0) = committee.members().get_entry_by_idx(0);
    assert!(!member0.blocklisted());
    let (validator1, member1) = committee.members().get_entry_by_idx(1);
    assert!(!member1.blocklisted());

    let eth_address0 = crypto::ecdsa_pub_key_to_eth_address(validator0);
    let eth_address1 = crypto::ecdsa_pub_key_to_eth_address(validator1);

    // Blocklist both
    let blocklist = message::create_blocklist_message(
        chain_ids::haneul_testnet(),
        0, // seq
        0, // type 0 is blocklist
        vector[eth_address0, eth_address1],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // Blocklist both reverse order
    let blocklist = message::create_blocklist_message(
        chain_ids::haneul_testnet(),
        0, // seq
        0, // type 0 is blocklist
        vector[eth_address1, eth_address0],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // val 0 is blocklisted
    let (_, blocked_member) = committee.members().get_entry_by_idx(0);
    assert!(blocked_member.blocklisted());
    // val 1 is too
    let (_, blocked_member) = committee.members().get_entry_by_idx(1);
    assert!(blocked_member.blocklisted());

    // unblocklist val1
    let blocklist = message::create_blocklist_message(
        chain_ids::haneul_testnet(),
        1, // seq, this is supposed to increment, but we don't test it here
        1, // type 1 is unblocklist
        vector[eth_address1],
    );
    let blocklist = message::extract_blocklist_payload(&blocklist);
    execute_blocklist(&mut committee, blocklist);

    // val 0 is still blocklisted
    let (_, blocked_member) = committee.members().get_entry_by_idx(0);
    assert!(blocked_member.blocklisted());
    // val 1 is not
    let (_, blocked_member) = committee.members().get_entry_by_idx(1);
    assert!(!blocked_member.blocklisted());

    // Clean up
    destroy(committee);
}

fun setup_test(): BridgeCommittee {
    let mut members = vec_map::empty<vector<u8>, CommitteeMember>();

    let bridge_pubkey_bytes = hex::decode(VALIDATOR1_PUBKEY);
    members.insert(
        bridge_pubkey_bytes,
        make_committee_member(
            @0xA,
            bridge_pubkey_bytes,
            3333,
            b"https://127.0.0.1:9191",
            false,
        ),
    );

    let bridge_pubkey_bytes = hex::decode(VALIDATOR2_PUBKEY);
    members.insert(
        bridge_pubkey_bytes,
        make_committee_member(
            @0xC,
            bridge_pubkey_bytes,
            3333,
            b"https://127.0.0.1:9192",
            false,
        ),
    );

    make_bridge_committee(members, vec_map::empty(), 1)
}
