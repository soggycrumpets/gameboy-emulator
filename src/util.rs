pub fn get_bit(byte: u8, bit: u8) -> bool {
    (byte & (1 << bit)) != 0
}

pub fn set_bit(byte: u8, bit: u8, set: bool) -> u8 {
    let mut result = byte & !(1 << bit); // Mask off the bit
    result |= (set as u8) << bit; // Set the bit
    result
}

