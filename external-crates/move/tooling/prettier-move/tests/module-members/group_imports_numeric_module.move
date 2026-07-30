// options:
// printWidth: 80
// useModuleLabel: true
// autoGroupImports: module

module prettier::group_imports_numeric_module;

use 0x2::{coin, transfer as t};
use 0x0::{Account::{Self, Account}, Something};
use 0x2::haneul::HANEUL;

fun f(_: Account, _: Something, _: HANEUL) {
    abort 0
}
