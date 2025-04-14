pub fn get_bit(byte: u8, index: u8) -> bool {
    (byte & (1 << index)) != 0
}

pub fn set_bit(byte: u8, index: u8, set: bool) -> u8 {
    let mut result = byte & !(1 << index); // Mask off the bit
    result |= (set as u8) << index; // Set the bit
    result
}

