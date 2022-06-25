use scrypto::prelude::*;

pub fn seed(min: u128, max: u128) -> u128 {
    let magnitude: u128 = max-min;
    let pseudorandom_number: u128 = Runtime::generate_uuid();
    let seed = pseudorandom_number % magnitude + min;
    seed
}

pub fn seed_decimal(min: u128, max: u128, float: Decimal) -> Decimal {
    let magnitude = max-min;
    let pseudorandom_number: u128 = Runtime::generate_uuid();
    let seed = pseudorandom_number % magnitude + min;
    let int = seed as i128;
    let dec: Decimal = int.into();
    dec / float
}