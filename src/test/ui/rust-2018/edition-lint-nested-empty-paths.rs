// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-rustfix

#![feature(rust_2018_preview, crate_visibility_modifier)]
#![deny(absolute_paths_not_starting_with_crate)]
#![allow(unused_imports)]
#![allow(dead_code)]

crate mod foo {
    crate mod bar {
        crate mod baz { }
        crate mod baz1 { }

        crate struct XX;
    }
}

use foo::{bar::{baz::{}}};
//~^ ERROR absolute paths must start with
//~| WARN this was previously accepted

use foo::{bar::{XX, baz::{}}};
//~^ ERROR absolute paths must start with
//~| WARN this was previously accepted

use foo::{bar::{baz::{}, baz1::{}}};
//~^ ERROR absolute paths must start with
//~| WARN this was previously accepted

fn main() {
}
