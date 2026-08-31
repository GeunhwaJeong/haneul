// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::RpcService;
use haneul_rpc::proto::haneul::rpc::v2::GetBalanceRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetBalanceResponse;
use haneul_rpc::proto::haneul::rpc::v2::GetCoinInfoRequest;
use haneul_rpc::proto::haneul::rpc::v2::GetCoinInfoResponse;
use haneul_rpc::proto::haneul::rpc::v2::ListBalancesRequest;
use haneul_rpc::proto::haneul::rpc::v2::ListBalancesResponse;
use haneul_rpc::proto::haneul::rpc::v2::ListDynamicFieldsRequest;
use haneul_rpc::proto::haneul::rpc::v2::ListDynamicFieldsResponse;
use haneul_rpc::proto::haneul::rpc::v2::ListOwnedObjectsRequest;
use haneul_rpc::proto::haneul::rpc::v2::ListOwnedObjectsResponse;
use haneul_rpc::proto::haneul::rpc::v2::state_service_server::StateService;

mod get_balance;
mod get_coin_info;
mod list_balances;
mod list_dynamic_fields;
mod list_owned_objects;

#[tonic::async_trait]
impl StateService for RpcService {
    async fn list_dynamic_fields(
        &self,
        request: tonic::Request<ListDynamicFieldsRequest>,
    ) -> Result<tonic::Response<ListDynamicFieldsResponse>, tonic::Status> {
        let service = self.clone();
        tokio::task::spawn_blocking(move || {
            list_dynamic_fields::list_dynamic_fields(&service, request.into_inner())
        })
        .await
        .map_err(|e| tonic::Status::internal(format!("list_dynamic_fields task failed: {e}")))?
        .map(tonic::Response::new)
        .map_err(Into::into)
    }

    async fn list_owned_objects(
        &self,
        request: tonic::Request<ListOwnedObjectsRequest>,
    ) -> Result<tonic::Response<ListOwnedObjectsResponse>, tonic::Status> {
        let service = self.clone();
        tokio::task::spawn_blocking(move || {
            list_owned_objects::list_owned_objects(&service, request.into_inner())
        })
        .await
        .map_err(|e| tonic::Status::internal(format!("list_owned_objects task failed: {e}")))?
        .map(tonic::Response::new)
        .map_err(Into::into)
    }

    async fn get_coin_info(
        &self,
        request: tonic::Request<GetCoinInfoRequest>,
    ) -> Result<tonic::Response<GetCoinInfoResponse>, tonic::Status> {
        get_coin_info::get_coin_info(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn get_balance(
        &self,
        request: tonic::Request<GetBalanceRequest>,
    ) -> Result<tonic::Response<GetBalanceResponse>, tonic::Status> {
        get_balance::get_balance(self, request.into_inner())
            .map(tonic::Response::new)
            .map_err(Into::into)
    }

    async fn list_balances(
        &self,
        request: tonic::Request<ListBalancesRequest>,
    ) -> Result<tonic::Response<ListBalancesResponse>, tonic::Status> {
        let service = self.clone();
        tokio::task::spawn_blocking(move || {
            list_balances::list_balances(&service, request.into_inner())
        })
        .await
        .map_err(|e| tonic::Status::internal(format!("list_balances task failed: {e}")))?
        .map(tonic::Response::new)
        .map_err(Into::into)
    }
}
