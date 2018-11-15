/*
AHHH
this does not work because we cannot box serde as a trait object.
There is an erased_serde crate, but I don't know that this will help with
what i am after..
*/
//use serde::Deserialize;
use std::fmt::{Debug, Display};
use std::collections::HashMap;

// we cannot constrain a box'ed trait object with multiple traits directly
// due to limitations with how the vtable will be set up.
// One way around this is to declare a trait with the appropriate constraints
// and then implement it trivially for a generic type.
pub trait DisplayDebug: Display + Debug {}
impl<T> DisplayDebug for T where T: Display + Debug {}

type HMap = HashMap<String, Box<DisplayDebug>>;

pub struct IndexMgr {
    pub mapping: HMap,
}

impl IndexMgr {
    pub fn new() -> Self {
        Self {
            mapping: HMap::new()
        }
    }
    /// insert a Boxed function object which implements Display and Debug
    pub fn add<I: Into<String>>(&mut self, key: I, value: Box<DisplayDebug>) {
        self.mapping.insert(key.into(), value);
    }
}