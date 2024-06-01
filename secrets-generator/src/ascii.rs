use std::ops::RangeInclusive;

pub fn get_lowercase_letter() -> RangeInclusive<i32> {
    return 97..=122;
}

pub fn get_uppercase_letter() -> RangeInclusive<i32> {
    return 65..=90;
}

pub fn get_number() -> RangeInclusive<i32> {
    return 48..=57;
}

pub fn get_special_character() -> [i32; 32] {
    let special_characters = [
        33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 58, 59, 60, 61, 62, 63, 64, 91,
        92, 93, 94, 95, 96, 123, 124, 125, 126,
    ];
    return special_characters;
}
