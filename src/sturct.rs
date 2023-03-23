use std::collections::BTreeMap;

#[derive(Debug)]
pub struct HypotheticalMessage {
    pub variable_string: String,
    pub an_enum: SomeType,
    pub small_int: u8,
    pub large_int: u32,
    pub nested: Option<Box<HypotheticalMessage>>,
    pub array: Vec<HypotheticalMessage>,
    pub map: BTreeMap<String, HypotheticalMessage>,
}

#[derive(Debug)]
pub enum SomeType {
    TypeOne,
    TypeTwo,
    TypeThree
}