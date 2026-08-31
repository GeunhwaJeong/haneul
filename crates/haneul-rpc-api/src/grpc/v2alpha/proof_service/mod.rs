// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use haneul_rpc::proto::haneul::rpc::v2alpha::GetCheckpointObjectProofRequest;
use haneul_rpc::proto::haneul::rpc::v2alpha::GetCheckpointObjectProofResponse;
use haneul_rpc::proto::haneul::rpc::v2alpha::proof_service_server::ProofService;

use crate::RpcService;

mod get_checkpoint_object_proof;

#[tonic::async_trait]
impl ProofService for RpcService {
    async fn get_checkpoint_object_proof(
        &self,
        request: tonic::Request<GetCheckpointObjectProofRequest>,
    ) -> Result<tonic::Response<GetCheckpointObjectProofResponse>, tonic::Status> {
        let response =
            get_checkpoint_object_proof::get_checkpoint_object_proof(self, request.into_inner())
                .map_err(tonic::Status::from)?;
        Ok(tonic::Response::new(response))
    }
}
