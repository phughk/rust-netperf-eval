use crate::sturct::HypotheticalMessage;
use crate::sturct::SomeType;

use rand::{distributions::Alphanumeric, Rng};

static STRING_SIZE: usize = 50;
static SMALL_SIZE: u8 = 255;
static LARGE_SIZE: u32 = 4294967295;

pub fn generate() -> HypotheticalMessage {
    let mut rng = rand::thread_rng();
    HypotheticalMessage {
        variable_string: rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(STRING_SIZE)
            .map(char::from)
            .collect(),
        an_enum: SomeType::TypeOne,
        small_int: rng.gen_range(0..SMALL_SIZE),
        large_int: rng.gen_range(0..LARGE_SIZE),
        nested: Option::Some(Box::new(HypotheticalMessage {
            variable_string: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(STRING_SIZE)
                .map(char::from)
                .collect(),
            an_enum: SomeType::TypeOne,
            small_int: rng.gen_range(0..SMALL_SIZE),
            large_int: rng.gen_range(0..LARGE_SIZE),
            nested: Option::None,
            array: vec![],
            map: Default::default(),
        })),
        array: vec![],
        map: Default::default(),
    }
}