//@aux-build:proc_macros.rs

#![warn(clippy::extern_without_abi)]

extern crate proc_macros;
use proc_macros::{external, with_span};

#[rustfmt::skip]
extern fn foo() {}
//~^ ERROR: `extern` missing explicit ABI

#[rustfmt::skip]
extern 	
fn foo_two() {}
//~^^ ERROR: `extern` missing explicit ABI

extern "C" fn bar() {}

#[rustfmt::skip]
extern 	
"C" 
fn bar_two() {}

extern "system" fn baz() {}

#[rustfmt::skip]
extern {
//~^ ERROR: `extern` missing explicit ABI
    fn qux();
}

#[rustfmt::skip]
extern 	
{
//~^^ ERROR: `extern` missing explicit ABI
    fn qux_two();
}

#[rustfmt::skip]
extern {fn qux_three();}
//~^ ERROR: `extern` missing explicit ABI

extern "C" {
    fn grault();
}

extern "system" {
    fn grault_two();
}

external! {
    extern fn waldo() {}
}

with_span! {
    span
    extern fn waldo_two() {}
}

fn main() {}
