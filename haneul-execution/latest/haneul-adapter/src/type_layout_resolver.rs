// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::data_store::cached_package_store::CachedPackageStore;
use crate::data_store::transaction_package_store::TransactionPackageStore;
use crate::static_programmable_transactions::linkage::config::{LinkageConfig, ResolutionConfig};
use crate::static_programmable_transactions::linkage::resolved_linkage::ExecutableLinkage;
use haneul_protocol_config::ProtocolConfig;
use haneul_types::TypeTag;
use haneul_types::base_types::ObjectID;
use haneul_types::error::{ExecutionError, HaneulErrorKind, HaneulResult};
use haneul_types::execution::TypeLayoutStore;
use haneul_types::storage::{BackingPackageStore, PackageObject};
use haneul_types::{error::HaneulError, layout_resolver::LayoutResolver};
use move_core_types::annotated_value as A;
use move_core_types::language_storage::StructTag;
use move_vm_runtime::runtime::MoveRuntime;

/// Retrieve a `MoveStructLayout` from a `Type`.
pub struct TypeLayoutResolver<'state, 'runtime> {
    vm: &'runtime MoveRuntime,
    protocol_config: &'runtime ProtocolConfig,
    state_view: Box<dyn TypeLayoutStore + 'state>,
}

/// Implements HaneulResolver traits by providing null implementations for module
/// resolution and delegating backing package resolution to the trait object.
struct NullHaneulResolver<'a, 'state>(&'a (dyn TypeLayoutStore + 'state));

impl<'state, 'runtime> TypeLayoutResolver<'state, 'runtime> {
    pub fn new(
        vm: &'runtime MoveRuntime,
        protocol_config: &'runtime ProtocolConfig,
        state_view: Box<dyn TypeLayoutStore + 'state>,
    ) -> Self {
        Self {
            vm,
            protocol_config,
            state_view,
        }
    }
}

impl LayoutResolver for TypeLayoutResolver<'_, '_> {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveDatatypeLayout, HaneulError> {
        let ids = struct_tag.all_addresses().into_iter().map(ObjectID::from);
        let null_resolver = NullHaneulResolver(&self.state_view);
        let resolver =
            CachedPackageStore::new(self.vm, TransactionPackageStore::new(&null_resolver));
        let config = ResolutionConfig::new(
            LinkageConfig::new(
                self.protocol_config
                    .include_special_package_amendments_as_option()
                    .clone(),
                true,
            ),
            self.protocol_config.binary_config(None),
        );
        let tag_linkage =
            ExecutableLinkage::type_linkage::<_, ExecutionError>(config, ids, &resolver)?;
        let link_context = tag_linkage.linkage_context::<ExecutionError>()?;
        let data_store = TransactionPackageStore::new(&null_resolver);
        let Ok(vm) = self.vm.make_vm(data_store, link_context) else {
            return Err(HaneulErrorKind::FailObjectLayout {
                st: format!("{}", struct_tag),
            }
            .into());
        };

        let type_tag = TypeTag::Struct(Box::new(struct_tag.clone()));
        match vm.annotated_type_layout(&type_tag) {
            Ok(A::MoveTypeLayout::Struct(s)) => Ok(A::MoveDatatypeLayout::Struct(s)),
            Ok(A::MoveTypeLayout::Enum(e)) => Ok(A::MoveDatatypeLayout::Enum(e)),
            _ => Err(HaneulErrorKind::FailObjectLayout {
                st: format!("{}", struct_tag),
            }
            .into()),
        }
    }
}

impl BackingPackageStore for NullHaneulResolver<'_, '_> {
    fn get_package_object(&self, package_id: &ObjectID) -> HaneulResult<Option<PackageObject>> {
        self.0.get_package_object(package_id)
    }
}
