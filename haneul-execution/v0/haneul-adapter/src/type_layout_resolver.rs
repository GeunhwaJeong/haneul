// Copyright (c) Mysten Labs, Inc.
// Modifications Copyright (c) 2026 Geunhwa Jeong
// SPDX-License-Identifier: Apache-2.0

use crate::programmable_transactions::context::new_session_for_linkage;
use crate::programmable_transactions::{
    context::load_type,
    linkage_view::{LinkageInfo, LinkageView},
};
use haneul_types::base_types::ObjectID;
use haneul_types::error::{HaneulErrorKind, HaneulResult};
use haneul_types::execution::TypeLayoutStore;
use haneul_types::storage::{BackingPackageStore, PackageObject};
use haneul_types::{error::HaneulError, layout_resolver::LayoutResolver};
use move_core_types::annotated_value as A;
use move_core_types::language_storage::{StructTag, TypeTag};
use move_vm_runtime::{move_vm::MoveVM, session::Session};

/// Retrieve a `MoveStructLayout` from a `Type`.
/// Invocation into the `Session` to leverage the `LinkageView` implementation
/// common to the runtime.
pub struct TypeLayoutResolver<'state, 'vm> {
    session: Session<'state, 'vm, LinkageView<'state>>,
}

/// Implements HaneulResolver traits by providing null implementations for module and resource
/// resolution and delegating backing package resolution to the trait object.
struct NullHaneulResolver<'state>(Box<dyn TypeLayoutStore + 'state>);

impl<'state, 'vm> TypeLayoutResolver<'state, 'vm> {
    pub fn new(vm: &'vm MoveVM, state_view: Box<dyn TypeLayoutStore + 'state>) -> Self {
        let session = new_session_for_linkage(
            vm,
            LinkageView::new(Box::new(NullHaneulResolver(state_view)), LinkageInfo::Unset),
        );
        Self { session }
    }
}

impl LayoutResolver for TypeLayoutResolver<'_, '_> {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveDatatypeLayout, HaneulError> {
        let type_tag: TypeTag = TypeTag::from(struct_tag.clone());
        let Ok(ty) = load_type(&mut self.session, &type_tag) else {
            return Err(HaneulErrorKind::FailObjectLayout {
                st: format!("{}", struct_tag),
            }
            .into());
        };
        let layout = self.session.type_to_fully_annotated_layout(&ty);
        let Ok(A::MoveTypeLayout::Struct(layout)) = layout else {
            return Err(HaneulErrorKind::FailObjectLayout {
                st: format!("{}", struct_tag),
            }
            .into());
        };
        Ok(A::MoveDatatypeLayout::Struct(layout))
    }
}

impl BackingPackageStore for NullHaneulResolver<'_> {
    fn get_package_object(&self, package_id: &ObjectID) -> HaneulResult<Option<PackageObject>> {
        self.0.get_package_object(package_id)
    }
}
