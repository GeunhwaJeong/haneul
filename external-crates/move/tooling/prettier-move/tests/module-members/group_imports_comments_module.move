// options:
// printWidth: 80
// useModuleLabel: true
// autoGroupImports: module

module prettier::group_imports_comments_module;

// stays in place in module mode too
use haneul::coin::Coin;
use haneul::{balance::Balance, haneul::HANEUL}; // kept with its comment
use std::string::String;
use std::ascii::String as ASCII;

fun f(_: Coin<u64>, _: Balance<u64>, _: HANEUL, _: String, _: ASCII) {
    abort 0
}
