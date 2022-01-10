#[allow(unused)]
use std::{collections::HashMap, fmt::Result};
#[allow(unused)]
use std::io::{self, Write};
#[allow(unused)]
// Brings all pub items defined in std::collections into scope.
use std::collections::*;
#[allow(unused)]
// brings second Result into scope as ioResult
use std::io::Result as ioResult;



fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("Hello, world!");
}
