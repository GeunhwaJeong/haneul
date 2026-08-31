// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::balance::Balance;
use crate::base_types::{MoveObjectType, TransactionDigest};
use crate::collection_types::VecMap;
use crate::dynamic_field::{derive_dynamic_field_id, serialize_dynamic_field};
use crate::haneul_system_state::haneul_system_state_inner_v1::{
    HaneulSystemStateInnerV1, StakeSubsidyV1, StorageFundV1, SystemParametersV1, ValidatorSetV1,
};
use crate::haneul_system_state::haneul_system_state_inner_v2::{
    HaneulSystemStateInnerV2, SystemParametersV2,
};
use crate::haneul_system_state::{HaneulSystemState, HaneulSystemStateWrapper};
use crate::id::UID;
use crate::object::{MoveObject, Object, Owner};
use crate::{HANEUL_SYSTEM_STATE_OBJECT_ID, MoveTypeTagTrait};
use haneul_protocol_config::ProtocolConfig;

pub fn validator_set_v1() -> ValidatorSetV1 {
    ValidatorSetV1 {
        total_stake: 0,
        active_validators: vec![],
        pending_active_validators: Default::default(),
        pending_removals: vec![],
        staking_pool_mappings: Default::default(),
        inactive_validators: Default::default(),
        validator_candidates: Default::default(),
        at_risk_validators: VecMap { contents: vec![] },
        extra_fields: Default::default(),
    }
}

pub fn storage_fund_v1() -> StorageFundV1 {
    StorageFundV1 {
        total_object_storage_rebates: Balance::new(0),
        non_refundable_balance: Balance::new(0),
    }
}

pub fn system_parameters_v1() -> SystemParametersV1 {
    SystemParametersV1 {
        epoch_duration_ms: 0,
        stake_subsidy_start_epoch: 0,
        max_validator_count: 0,
        min_validator_joining_stake: 0,
        validator_low_stake_threshold: 0,
        validator_very_low_stake_threshold: 0,
        validator_low_stake_grace_period: 0,
        extra_fields: Default::default(),
    }
}

pub fn stake_subsidy_v1() -> StakeSubsidyV1 {
    StakeSubsidyV1 {
        balance: Balance::new(0),
        distribution_counter: 0,
        current_distribution_amount: 0,
        stake_subsidy_period_length: 0,
        stake_subsidy_decrease_rate: 0,
        extra_fields: Default::default(),
    }
}

pub fn haneul_system_state_inner_v1() -> HaneulSystemStateInnerV1 {
    HaneulSystemStateInnerV1 {
        epoch: 0,
        protocol_version: 0,
        // must be 1
        system_state_version: 1,
        validators: validator_set_v1(),
        storage_fund: storage_fund_v1(),
        parameters: system_parameters_v1(),
        reference_gas_price: 0,
        validator_report_records: VecMap { contents: vec![] },
        stake_subsidy: stake_subsidy_v1(),
        safe_mode: false,
        safe_mode_storage_rewards: Balance::new(0),
        safe_mode_computation_rewards: Balance::new(0),
        safe_mode_storage_rebates: 0,
        safe_mode_non_refundable_storage_fee: 0,
        epoch_start_timestamp_ms: 0,
        extra_fields: Default::default(),
    }
}

pub fn system_parameters_v2() -> SystemParametersV2 {
    SystemParametersV2 {
        epoch_duration_ms: 0,
        stake_subsidy_start_epoch: 0,
        min_validator_count: 0,
        max_validator_count: 0,
        min_validator_joining_stake: 0,
        validator_low_stake_threshold: 0,
        validator_very_low_stake_threshold: 0,
        validator_low_stake_grace_period: 0,
        extra_fields: Default::default(),
    }
}

pub fn haneul_system_state_inner_v2() -> HaneulSystemStateInnerV2 {
    HaneulSystemStateInnerV2 {
        epoch: 0,
        protocol_version: 0,
        // must be 2
        system_state_version: 2,
        validators: validator_set_v1(),
        storage_fund: storage_fund_v1(),
        parameters: system_parameters_v2(),
        reference_gas_price: 0,
        validator_report_records: VecMap { contents: vec![] },
        stake_subsidy: stake_subsidy_v1(),
        safe_mode: false,
        safe_mode_storage_rewards: Balance::new(0),
        safe_mode_computation_rewards: Balance::new(0),
        safe_mode_storage_rebates: 0,
        safe_mode_non_refundable_storage_fee: 0,
        epoch_start_timestamp_ms: 0,
        extra_fields: Default::default(),
    }
}

pub fn system_state_output_objects(haneul_system_state: HaneulSystemState) -> Vec<Object> {
    let version = haneul_system_state.version();
    let system_state_wrapper_object = Object::new_move(
        unsafe {
            MoveObject::new_from_execution(
                MoveObjectType::gas_coin(),
                // must be true to pass validation
                true,
                0.into(),
                bcs::to_bytes(&HaneulSystemStateWrapper {
                    // must be HANEUL_SYSTEM_STATE_OBJECT_ID
                    id: UID::new(HANEUL_SYSTEM_STATE_OBJECT_ID),
                    version,
                })
                .unwrap(),
                &ProtocolConfig::get_for_max_version_UNSAFE(),
                false,
            )
            .unwrap()
        },
        Owner::Immutable,
        TransactionDigest::genesis_marker(),
    );
    let dynamic_field_key = 0u64; // value does not matter

    // hash based on system_state_wrapper_object
    let system_state_object_id = derive_dynamic_field_id(
        HANEUL_SYSTEM_STATE_OBJECT_ID,
        &dynamic_field_key.get_instance_type_tag(),
        &bcs::to_bytes(&version).unwrap(),
    )
    .unwrap();

    let field_id = UID::new(system_state_object_id);
    let object_bytes = match haneul_system_state {
        HaneulSystemState::V1(inner) => {
            serialize_dynamic_field(&field_id, &dynamic_field_key, inner)
        }
        HaneulSystemState::V2(inner) => {
            serialize_dynamic_field(&field_id, &dynamic_field_key, inner)
        }
        #[cfg(msim)]
        HaneulSystemState::SimTestV1(_)
        | HaneulSystemState::SimTestShallowV2(_)
        | HaneulSystemState::SimTestDeepV2(_) => unimplemented!(),
    }
    .unwrap();
    let system_state_inner_object = Object::new_move(
        unsafe {
            MoveObject::new_from_execution(
                MoveObjectType::gas_coin(),
                true,
                0.into(),
                object_bytes,
                &ProtocolConfig::get_for_max_version_UNSAFE(),
                false,
            )
            .unwrap()
        },
        Owner::Immutable,
        TransactionDigest::genesis_marker(),
    );
    vec![system_state_wrapper_object, system_state_inner_object]
}
