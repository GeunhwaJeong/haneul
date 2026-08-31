// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::data_store::cached_package_store::CachedPackageStore;
use crate::data_store::legacy::linkage_view::LinkageView;
use crate::programmable_transactions::context::load_type_from_struct;
use haneul_types::base_types::ObjectID;
use haneul_types::error::{HaneulErrorKind, HaneulResult};
use haneul_types::execution::TypeLayoutStore;
use haneul_types::storage::{BackingPackageStore, PackageObject};
use haneul_types::{error::HaneulError, layout_resolver::LayoutResolver};
use move_core_types::annotated_value as A;
use move_core_types::language_storage::StructTag;
use move_vm_runtime::move_vm::MoveVM;

/// Retrieve a `MoveStructLayout` from a `Type`.
/// Invocation into the `Session` to leverage the `LinkageView` implementation
/// common to the runtime.
pub struct TypeLayoutResolver<'state, 'vm> {
    vm: &'vm MoveVM,
    linkage_view: LinkageView<'state>,
}

/// Implements HaneulResolver traits by providing null implementations for module and resource
/// resolution and delegating backing package resolution to the trait object.
struct NullHaneulResolver<'state>(Box<dyn TypeLayoutStore + 'state>);

impl<'state, 'vm> TypeLayoutResolver<'state, 'vm> {
    pub fn new(vm: &'vm MoveVM, state_view: Box<dyn TypeLayoutStore + 'state>) -> Self {
        let linkage_view = LinkageView::new(Box::new(CachedPackageStore::new(Box::new(
            NullHaneulResolver(state_view),
        ))));
        Self { vm, linkage_view }
    }
}

impl LayoutResolver for TypeLayoutResolver<'_, '_> {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveDatatypeLayout, HaneulError> {
        let Ok(ty) = load_type_from_struct(self.vm, &self.linkage_view, &[], struct_tag) else {
            return Err(HaneulErrorKind::FailObjectLayout {
                st: format!("{}", struct_tag),
            }
            .into());
        };
        let layout = self.vm.get_runtime().type_to_fully_annotated_layout(&ty);
        match layout {
            Ok(A::MoveTypeLayout::Struct(s)) => Ok(A::MoveDatatypeLayout::Struct(s)),
            Ok(A::MoveTypeLayout::Enum(e)) => Ok(A::MoveDatatypeLayout::Enum(e)),
            _ => Err(HaneulErrorKind::FailObjectLayout {
                st: format!("{}", struct_tag),
            }
            .into()),
        }
    }
}

impl BackingPackageStore for NullHaneulResolver<'_> {
    fn get_package_object(&self, package_id: &ObjectID) -> HaneulResult<Option<PackageObject>> {
        self.0.get_package_object(package_id)
    }
}
