// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// run-pass
// pretty-expanded FIXME #23616

#![deny(warnings)]
#![allow(unused_imports)]

pub enum Foo { A }
mod bar {
    pub fn normal(x: ::Foo) {
        use Foo::A;
        match x {
            A => {}
        }
    }
    pub fn wrong(x: ::Foo) {
        match x {
            ::Foo::A => {}
        }
    }
}

pub fn main() {
    bar::normal(Foo::A);
    bar::wrong(Foo::A);
}
