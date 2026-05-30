mod prefixed_instructions;
mod unprefixed_instructions;

#[derive(Debug)]
pub struct Instruction<'a> {
    mnemonic: &'a str,
    bytes: u8,
    cycles: u8,
    operands: [&'a str; 2],
}