pub fn get_bit(byte: u8, index: u8) -> bool {
    (byte & (1 << index)) != 0
}

pub fn set_bit(byte: &mut u8, index: u8, set: bool) {
    let mut result = *byte & !(1 << index);
    result |= (set as u8) << index;
    *byte = result;
}
