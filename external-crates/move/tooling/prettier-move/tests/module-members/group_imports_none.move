// options:
// printWidth: 80
// useModuleLabel: true
// autoGroupImports: none

module prettier::group_imports_none;

use haneul::coin::Coin;
use std::string::String;
use haneul::balance::Balance; // trailing comment stays
#[test_only]
use haneul::test_scenario;
use haneul::{
    // comment inside braces still breaks them
    clock::Clock,
    table::Table,
};
use haneul::{vec_map::VecMap, vec_set::VecSet};

fun f(_: Coin<u64>, _: String, _: Balance<u64>, _: &Clock, _: Table<u8, u8>) {
    abort 0
}
