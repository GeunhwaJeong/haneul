// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::RpcService;
use haneul_rpc::proto::haneul::rpc::v2::BatchGetObjectsRequest;
use haneul_rpc::proto::haneul::rpc::v2::BatchGetObjectsResponse;
use haneul_rpc::proto::haneul::rpc::v2::BatchGetTransactionsRequest;
use haneul_rpc::proto::haneul::rpc::v2::BatchGetTransactionsResponse;
use haneul_rpc::proto::haneul::rpc::v2::GetCheckpointRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetCheckpointResponse;
use haneul_rpc::proto::haneul::rpc::v2::GetEpochRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetEpochResponse;
use haneul_rpc::proto::haneul::rpc::v2::GetObjectRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetObjectResponse;
use haneul_rpc::proto::haneul::rpc::v2::GetServiceInfoRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetServiceInfoResponse;
use haneul_rpc::proto::haneul::rpc::v2::GetTransactionRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetTransactionResponse;
use haneul_rpc::proto::haneul::rpc::v2::ListCheckpointsRequest;
use haneul_rpc::proto::haneul::rpc::v2::ListCheckpointsResponse;
use haneul_rpc::proto::haneul::rpc::v2::ListEventsRequest;
use haneul_rpc::proto::haneul::rpc::v2::ListEventsResponse;
use haneul_rpc::proto::haneul::rpc::v2::ListTransactionsRequest;
use haneul_rpc::proto::haneul::rpc::v2::ListTransactionsResponse;
use haneul_rpc::proto::haneul::rpc::v2::ledger_service_server::LedgerService;
use tonic::codegen::BoxStream;

mod bitmap_scan;
mod chunked_scan;
mod event_scan;
pub(crate) mod get_checkpoint;
mod get_epoch;
mod get_object;
mod get_service_info;
pub(crate) mod get_transaction;
mod ledger_read;
mod list_checkpoints;
mod list_events;
mod list_transactions;
mod object_set;
mod query_end;
mod stream;
pub use get_epoch::protocol_config_to_proto;
pub use get_object::validate_get_object_requests;

use stream::serve_list_stream;

#[tonic::async_trait]
impl LedgerService for RpcService {
    async fn get_service_info(
        &self,
        _request: tonic::Request<GetServiceInfoRequest>,
    ) -> Result<tonic::Response<GetServiceInfoResponse>, tonic::Status> {
        get_service_info::get_service_info(self)
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_object(
        &self,
        request: tonic::Request<GetObjectRequest>,
    ) -> Result<tonic::Response<GetObjectResponse>, tonic::Status> {
        get_object::get_object(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn batch_get_objects(
        &self,
        request: tonic::Request<BatchGetObjectsRequest>,
    ) -> Result<tonic::Response<BatchGetObjectsResponse>, tonic::Status> {
        get_object::batch_get_objects(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_transaction(
        &self,
        request: tonic::Request<GetTransactionRequest>,
    ) -> Result<tonic::Response<GetTransactionResponse>, tonic::Status> {
        get_transaction::get_transaction(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn batch_get_transactions(
        &self,
        request: tonic::Request<BatchGetTransactionsRequest>,
    ) -> Result<tonic::Response<BatchGetTransactionsResponse>, tonic::Status> {
        get_transaction::batch_get_transactions(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_checkpoint(
        &self,
        request: tonic::Request<GetCheckpointRequest>,
    ) -> Result<tonic::Response<GetCheckpointResponse>, tonic::Status> {
        get_checkpoint::get_checkpoint(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_epoch(
        &self,
        request: tonic::Request<GetEpochRequest>,
    ) -> Result<tonic::Response<GetEpochResponse>, tonic::Status> {
        get_epoch::get_epoch(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn list_checkpoints(
        &self,
        request: tonic::Request<ListCheckpointsRequest>,
    ) -> Result<tonic::Response<BoxStream<ListCheckpointsResponse>>, tonic::Status> {
        serve_list_stream(
            "list_checkpoints",
            self.config.ledger_history().list_checkpoints().timeout,
            list_checkpoints::list_checkpoints(self.clone(), request.into_inner()),
        )
        .await
    }

    async fn list_transactions(
        &self,
        request: tonic::Request<ListTransactionsRequest>,
    ) -> Result<tonic::Response<BoxStream<ListTransactionsResponse>>, tonic::Status> {
        serve_list_stream(
            "list_transactions",
            self.config.ledger_history().list_transactions().timeout,
            list_transactions::list_transactions(self.clone(), request.into_inner()),
        )
        .await
    }

    async fn list_events(
        &self,
        request: tonic::Request<ListEventsRequest>,
    ) -> Result<tonic::Response<BoxStream<ListEventsResponse>>, tonic::Status> {
        serve_list_stream(
            "list_events",
            self.config.ledger_history().list_events().timeout,
            list_events::list_events(self.clone(), request.into_inner()),
        )
        .await
    }
}
