use staquia::implementations::text::NumberSystem;

pub fn alphabetical_system() -> NumberSystem {
    NumberSystem {
        zero: '!',
        zero_code: 33,
        zero_code_i32: 33,
        first: 'a',
        first_code: 97,
        first_code_i32: 97,
        last: 'z',
        k: 0.15,
        base: 27,
        length_n: 4,
        length_z: 12,
        length: 16,
        length_k: 14,
    }
}
