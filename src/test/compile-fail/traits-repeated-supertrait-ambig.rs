// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Test a case of a trait which extends the same supertrait twice, but
// with difference type parameters. Test then that when we don't give
// enough information to pick between these, no selection is made. In
// this particular case, the two choices are i64/u64 -- so when we use
// an integer literal, we wind up falling this literal back to i32.
// See also `run-pass/trait-repeated-supertrait.rs`.

trait CompareTo<T> {
    fn same_as(&self, t: T) -> bool;
}

trait CompareToInts : CompareTo<i64> + CompareTo<u64> {
}

impl CompareTo<i64> for i64 {
    fn same_as(&self, t: i64) -> bool { *self == t }
}

impl CompareTo<u64> for i64 {
    fn same_as(&self, t: u64) -> bool { *self == (t as i64) }
}

impl CompareToInts for i64 { }

fn with_obj(c: &CompareToInts) -> bool {
    c.same_as(22) //~ ERROR `CompareTo<i32>` is not implemented
}

fn with_trait<C:CompareToInts>(c: &C) -> bool {
    c.same_as(22) //~ ERROR `CompareTo<i32>` is not implemented
}

fn with_ufcs1<C:CompareToInts>(c: &C) -> bool {
    CompareToInts::same_as(c, 22) //~ ERROR `CompareTo<i32>` is not implemented
}

fn with_ufcs2<C:CompareToInts>(c: &C) -> bool {
    CompareTo::same_as(c, 22) //~ ERROR `CompareTo<i32>` is not implemented
}

fn main() {
    assert_eq!(22_i64.same_as(22), true); //~ ERROR `CompareTo<i32>` is not implemented
}
