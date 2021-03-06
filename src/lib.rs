#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![allow(non_snake_case)]

mod acmd;
mod custom;

#[skyline::main(name = "EveryoneHasWitchTime")]
pub fn main() {
    acmd::install();
    custom::install();
}
