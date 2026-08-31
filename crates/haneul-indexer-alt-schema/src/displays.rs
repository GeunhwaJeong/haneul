// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use haneul_field_count::FieldCount;

use crate::schema::sum_displays;

#[derive(Insertable, Debug, Clone, FieldCount, Queryable)]
#[diesel(table_name = sum_displays, primary_key(object_type))]
pub struct StoredDisplay {
    pub object_type: Vec<u8>,
    pub display_id: Vec<u8>,
    pub display_version: i16,
    pub display: Vec<u8>,
}
