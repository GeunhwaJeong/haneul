// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

//! Rust bindings for `haneul::allowance`.
//!
//! At signing, a transaction's declared (funder, allowance) pair is checked against the loaded
//! `Allowance` object, and the withdrawal is reserved against the funder's balance. At execution,
//! the adapter mints the `AllowanceWithdrawal<T>` that the allowance's spend paths consume.

use crate::HANEUL_FRAMEWORK_ADDRESS;
use crate::base_types::HaneulAddress;
use crate::error::{UserInputError, UserInputResult};
use crate::id::UID;
use crate::object::Object;
use haneullabs_common::debug_fatal;
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use move_core_types::identifier::IdentStr;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_core_types::u256::U256;
use serde::{Deserialize, Serialize};

pub const ALLOWANCE_MODULE_NAME: &IdentStr = ident_str!("allowance");
pub const ALLOWANCE_STRUCT_NAME: &IdentStr = ident_str!("Allowance");
pub const ALLOWANCE_WITHDRAWAL_STRUCT_NAME: &IdentStr = ident_str!("AllowanceWithdrawal");
pub const RESOLVED_ALLOWANCE_WITHDRAWAL_STRUCT: (&AccountAddress, &IdentStr, &IdentStr) = (
    &HANEUL_FRAMEWORK_ADDRESS,
    ALLOWANCE_MODULE_NAME,
    ALLOWANCE_WITHDRAWAL_STRUCT_NAME,
);

/// Mirror of the Move struct `haneul::allowance::Allowance<T>`.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Allowance {
    pub id: UID,
    pub settings: Settings,
    pub current_spend: U256,
}

/// Mirror of the Move struct `haneul::allowance::Settings`.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Settings {
    pub funder: HaneulAddress,
    pub spender: Option<HaneulAddress>,
    /// `Option<std::type_name::TypeName>`.
    pub app: Option<String>,
    pub lifetime_cap: Option<U256>,
    pub start_timestamp_ms: Option<u64>,
    pub expiration_timestamp_ms: Option<u64>,
    pub rate_limit: Option<RateLimit>,
    pub name: String,
}

/// Mirror of the Move enum `haneul::allowance::RateLimit`.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum RateLimit {
    Windowed {
        limit: U256,
        spent: U256,
        anchor_ms: Option<u64>,
        index: u64,
        window: Window,
    },
}

/// Mirror of the Move enum `haneul::allowance::Window`.
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Window {
    PeriodicMs(u64),
    CalendarMonths(u8),
}

impl Allowance {
    pub fn type_(type_param: TypeTag) -> StructTag {
        StructTag {
            address: HANEUL_FRAMEWORK_ADDRESS,
            module: ALLOWANCE_MODULE_NAME.to_owned(),
            name: ALLOWANCE_STRUCT_NAME.to_owned(),
            type_params: vec![type_param],
        }
    }

    pub fn is_allowance(s: &StructTag) -> bool {
        s.address == HANEUL_FRAMEWORK_ADDRESS
            && s.module.as_ident_str() == ALLOWANCE_MODULE_NAME
            && s.name.as_ident_str() == ALLOWANCE_STRUCT_NAME
            && s.type_params.len() == 1
    }
}

/// Sign-time view of an `Allowance<T>`.
/// NB: The spender can rotate from transaction to transaction.
#[derive(Debug, Clone)]
pub struct ResolvedAllowance {
    pub funder: HaneulAddress,
    pub spender: Option<HaneulAddress>,
    /// The accumulated type `T` of `Allowance<T>` (e.g. `Balance<HANEUL>`).
    pub funds_type: TypeTag,
}

/// Parses an object as an `Allowance`, extracting the sign-time-relevant fields.
pub fn parse_allowance_object(object: &Object) -> UserInputResult<ResolvedAllowance> {
    let invalid = |error: String| UserInputError::InvalidWithdrawReservation { error };
    let id = object.id();
    let Some(move_obj) = object.data.try_as_move() else {
        return Err(invalid(format!(
            "Specified allowance {id} is not a Move object"
        )));
    };
    let tag: StructTag = move_obj.type_().clone().into();
    if !Allowance::is_allowance(&tag) {
        return Err(invalid(format!(
            "Specified allowance {id} is not a haneul::allowance::Allowance"
        )));
    }
    if !object.owner.is_shared() {
        return Err(invalid(format!("Allowance {id} is not a shared object")));
    }
    let funds_type = tag
        .type_params
        .into_iter()
        .next()
        .expect("checked by is_allowance");

    let Ok(allowance) = bcs::from_bytes::<Allowance>(move_obj.contents()) else {
        debug_fatal!("allowance {id} did not match the `Allowance` rust type");
        return Err(invalid(format!("Failed to read allowance {id}")));
    };

    Ok(ResolvedAllowance {
        funder: allowance.settings.funder,
        spender: allowance.settings.spender,
        funds_type,
    })
}
