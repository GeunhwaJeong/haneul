// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context as _;
use haneul_indexer_alt_reader::consistent_reader::proto::owner::OwnerKind;
use haneul_json_rpc_types::HaneulObjectDataOptions;
use haneul_json_rpc_types::Page as PageResponse;
use haneul_types::HANEUL_FRAMEWORK_ADDRESS;
use haneul_types::Identifier;
use haneul_types::base_types::HaneulAddress;
use haneul_types::base_types::ObjectID;
use haneul_types::dynamic_field::DYNAMIC_FIELD_FIELD_STRUCT_NAME;
use haneul_types::dynamic_field::DYNAMIC_FIELD_MODULE_NAME;
use haneul_types::haneul_serde::HaneulStructTag;
use move_core_types::language_storage::StructTag;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use serde_with::serde_as;

use crate::api::objects::error::Error;
use crate::context::Context;
use crate::error::RpcError;
use crate::error::invalid_params;
use crate::paginate::BcsCursor;
use crate::paginate::Cursor as _;
use crate::paginate::Page;

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase", rename = "ObjectResponseQuery", default)]
pub(crate) struct HaneulObjectResponseQuery {
    /// If None, no filter will be applied
    pub filter: Option<HaneulObjectDataFilter>,
    /// config which fields to include in the response, by default only digest is included
    pub options: Option<HaneulObjectDataOptions>,
}

#[serde_as]
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub(crate) enum HaneulObjectDataFilter {
    /// Query for object's that don't match any of these filters.
    MatchNone(Vec<HaneulObjectDataFilter>),

    /// Query by the object type's package.
    Package(ObjectID),
    /// Query by the object type's module.
    MoveModule {
        /// The package that contains the module.
        package: ObjectID,
        /// The module name.
        #[schemars(with = "String")]
        module: Identifier,
    },
    /// Query by the object's type.
    StructType(
        #[serde_as(as = "HaneulStructTag")]
        #[schemars(with = "String")]
        StructTag,
    ),
}

pub(crate) type Cursor = BcsCursor<Vec<u8>>;
pub(crate) type ObjectIDs = PageResponse<ObjectID, String>;

/// Fetch ObjectIDs for a page of objects owned by `owner` that satisfy the given `filter` and
/// pagination parameters. Returns the IDs and a cursor pointing to the last result (if there are
/// any results).
pub(super) async fn owned_objects(
    ctx: &Context,
    owner: HaneulAddress,
    filter: &Option<HaneulObjectDataFilter>,
    cursor: Option<String>,
    limit: Option<usize>,
) -> Result<ObjectIDs, RpcError<Error>> {
    match filter {
        // Limit filter to a single exclusion
        Some(HaneulObjectDataFilter::MatchNone(exclusions))
            if exclusions.len() == 1
                && !matches!(&exclusions[0], HaneulObjectDataFilter::MatchNone(_)) =>
        {
            query_objects(
                ctx,
                owner,
                OwnerKind::Address,
                Some(format!("!{}", filter_to_type_string(&exclusions[0]))),
                cursor,
                limit,
            )
            .await
        }
        Some(HaneulObjectDataFilter::MatchNone(_)) => {
            Err(invalid_params(Error::MultipleExclusions))
        }
        filter => {
            query_objects(
                ctx,
                owner,
                OwnerKind::Address,
                filter.as_ref().map(filter_to_type_string),
                cursor,
                limit,
            )
            .await
        }
    }
}

/// Fetch ObjectIDs for a page of dynamic fields owned by parent object `owner`. The returned IDs
/// all point to `haneul::dynamic_field::Field<K, V>` objects. Returns the IDs and a cursor pointing
/// to the last result (if there are any results).
pub(crate) async fn dynamic_fields(
    ctx: &Context,
    owner: ObjectID,
    cursor: Option<String>,
    limit: Option<usize>,
) -> Result<ObjectIDs, RpcError<Error>> {
    let type_ = StructTag {
        address: HANEUL_FRAMEWORK_ADDRESS,
        module: DYNAMIC_FIELD_MODULE_NAME.to_owned(),
        name: DYNAMIC_FIELD_FIELD_STRUCT_NAME.to_owned(),
        type_params: vec![],
    };

    query_objects(
        ctx,
        owner.into(),
        OwnerKind::Object,
        Some(type_.to_canonical_string(true)),
        cursor,
        limit,
    )
    .await
}

fn filter_to_type_string(filter: &HaneulObjectDataFilter) -> String {
    match filter {
        HaneulObjectDataFilter::Package(p) => p.to_string(),
        HaneulObjectDataFilter::MoveModule { package, module } => format!("{package}::{module}"),
        HaneulObjectDataFilter::StructType(tag) => tag.to_canonical_string(true),
        HaneulObjectDataFilter::MatchNone(_) => unreachable!(),
    }
}

async fn query_objects(
    ctx: &Context,
    owner: HaneulAddress,
    kind: OwnerKind,
    object_type: Option<String>,
    cursor: Option<String>,
    limit: Option<usize>,
) -> Result<ObjectIDs, RpcError<Error>> {
    let config = &ctx.config().objects;
    let page: Page<Cursor> = Page::from_params(
        config.default_page_size,
        config.max_page_size,
        cursor,
        limit,
        None,
    )?;

    if kind != OwnerKind::Address && kind != OwnerKind::Object {
        return Ok(PageResponse {
            data: vec![],
            next_cursor: None,
            has_next_page: false,
        });
    }

    let results = ctx
        .consistent_reader()
        .list_owned_objects(
            None,
            kind,
            Some(owner.to_string()),
            object_type,
            Some(page.limit as u32),
            page.cursor.as_ref().map(|c| c.0.clone()),
            None,
            true,
        )
        .await
        .context("Failed to list owned objects")?;

    let obj_ids = results
        .results
        .iter()
        .map(|obj_ref| obj_ref.value.0)
        .collect::<Vec<_>>();

    let next_cursor = results
        .results
        .last()
        .map(|edge| BcsCursor(edge.token.clone()).encode())
        .transpose()
        .context("Failed to encode cursor")?;

    Ok(PageResponse {
        data: obj_ids,
        next_cursor,
        has_next_page: results.has_next_page,
    })
}
