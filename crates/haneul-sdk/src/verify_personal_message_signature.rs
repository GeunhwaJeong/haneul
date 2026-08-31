// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use crate::error::Error;
use haneul_rpc::Client;
use haneul_rpc::proto::haneul::rpc::v2::{Bcs, UserSignature, VerifySignatureRequest};
use haneul_types::{
    base_types::HaneulAddress,
    signature::{AuthenticatorTrait, GenericSignature, VerifyParams},
    signature_verification::VerifiedDigestCache,
};
use shared_crypto::intent::{Intent, IntentMessage, PersonalMessage};

/// Verify a signature against a personal message bytes and the haneul address.
/// HaneulClient is required to pass in if zkLogin signature is supplied.
pub async fn verify_personal_message_signature(
    signature: GenericSignature,
    message: &[u8],
    address: HaneulAddress,
    client: Option<Client>,
) -> Result<(), Error> {
    let intent_msg = IntentMessage::new(
        Intent::personal_message(),
        PersonalMessage {
            message: message.to_vec(),
        },
    );
    match signature {
        GenericSignature::ZkLoginAuthenticator(ref _sig) => {
            if let Some(mut client) = client {
                let message = Bcs::serialize(&message)?.with_name("PersonalMessage");
                let user_signature =
                    UserSignature::default().with_bcs(Bcs::from(signature.as_ref().to_owned()));

                let res = client
                    .signature_verification_client()
                    .verify_signature(
                        VerifySignatureRequest::default()
                            .with_address(address.to_string())
                            .with_message(message)
                            .with_signature(user_signature),
                    )
                    .await
                    .map_err(|_| Error::InvalidSignature)?
                    .into_inner();

                if res.is_valid() {
                    Ok(())
                } else {
                    Err(Error::InvalidSignature)
                }
            } else {
                Err(Error::InvalidSignature)
            }
        }
        _ => signature
            .verify_claims::<PersonalMessage>(
                &intent_msg,
                address,
                &VerifyParams::default(),
                Arc::new(VerifiedDigestCache::new_empty()),
            )
            .map_err(|_| Error::InvalidSignature),
    }
}
