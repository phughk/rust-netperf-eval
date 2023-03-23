use std::collections::BTreeMap;

struct HypotheticalMessage {
    variable_string: String,
    an_enum: SomeType,
    small_int: u8,
    large_int: u32,
    nested: Box<HypotheticalMessage>,
    array: Vec<HypotheticalMessage>,
    map: BTreeMap<String, HypotheticalMessage>,
}

enum SomeType {
    TypeOne,
    TypeTwo,
    TypeThree
}