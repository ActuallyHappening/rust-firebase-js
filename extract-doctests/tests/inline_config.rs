#![allow(dead_code)]

use extract_doctests::extract_doctests;

#[extract_doctests(inline_config(template = ""))]
fn placeholder1() {}

#[extract_doctests(inline_config(template = "", replace_package = ""))]
fn placeholder2() {}

fn main() {}