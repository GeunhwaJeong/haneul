// options:
// printWidth: 40
// useModuleLabel: true
// autoGroupImports: module

module prettier::group_imports;

use a::b::{
    Self as c,
    e as f,
    g as f,
    h as i,
};
use std::ascii::String as ASCII;
use std::option::{Self as opt, Option};
use std::string::String;
use std::type_name::get as type_name_get;
use std::vector as vec;
use haneul::balance::{Self, Balance};
use haneul::coin::{Self, Coin};
use haneul::dynamic_field as df;
use haneul::dynamic_object_field as dof;
use haneul::event;
use haneul::haneul::HANEUL;
use haneul::transfer_policy::{
    Self,
    TransferPolicy,
    TransferRequest,
};

public fun do_something() {}
