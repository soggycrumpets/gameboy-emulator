use crate::prelude::*;

// use derive_quote_to_tokens::ToTokens;
use proc_macro2::TokenStream;
use quote::{TokenStreamExt, format_ident, quote};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use tracing::{Level, event, instrument};

const TEMP_FILENAME: &str = "../src/gen_opcodes_tmp.rs";
const GEN_FILENAME: &str = "../src/gen_opcodes.rs";

#[derive(Deserialize, Serialize, Debug)]
struct InputOperand {
    name: String,
    immediate: Option<bool>,
    bytes: Option<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct InputFlag {
    Z: String,
    N: String,
    H: String,
    C: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct InputOpcode {
    name: Option<String>,
    mnemonic: String,
    bytes: u8,
    cycles: Vec<u8>,
    operands: Vec<InputOperand>,
    immediate: bool,
    flags: InputFlag,
}


/// Use serde() to parse the json file.
fn deserialize_opcodes(values: &serde_json::Map<String, Value>) -> Result<Vec<InputOpcode>, Error> {
    let mut opcodes: Vec<InputOpcode> = Vec::new();
    for (key, value) in values {
        event!(Level::TRACE, "Key: {:?} Got json: {:?}\n", key, value);
        let mut op_code: InputOpcode = serde_json::from_value(value.clone())?;
        op_code.name = Some(key.to_string());
        event!(Level::INFO, "Parsed {:?}\n", op_code);
        opcodes.push(op_code);
    }
    Ok(opcodes)
}

fn generate_flags(flags: &InputFlag) -> TokenStream {
    let z = flags.Z.clone();
    let n = flags.N.clone();
    let h = flags.H.clone();
    let c = flags.C.clone();

    quote! {
        Flag {
            z: #z,
            n: #n,
            h: #h,
            c: #c,
        }
    }
}

fn generate_operands(input_operands: &[InputOperand]) -> TokenStream {
    let mut tokens = TokenStream::new();
    for op in input_operands.iter() {
        let name = op.name.clone();
        let immediate = op.immediate.unwrap_or(false);
        let bytes = op.bytes.unwrap_or(0);
        tokens.append_all(quote! {
                Operand {
                name: #name,
                immediate: #immediate,
                bytes: #bytes,
            },
        });
    }
    tokens
}

fn generate_opcode(input_opcode: &InputOpcode) -> TokenStream {
    let operand_tokens = generate_operands(&input_opcode.operands);
    let name = input_opcode.name.clone().unwrap();
    let mnemonic = input_opcode.mnemonic.clone();
    let bytes: u8 = input_opcode.bytes;
    let cycles = input_opcode.cycles.clone();
    let immediate = input_opcode.immediate;
    let flag_tokens = generate_flags(&input_opcode.flags);

    // let flags = convert_input_flags(&input_opcode.flags);
    quote! {
        Opcode {
            name: #name,
            mnemonic: #mnemonic,
            bytes: #bytes,
            cycles: vec! ( #(#cycles),* ),
            operands: vec! ( #operand_tokens )  ,
            immediate: #immediate,
            flags: #flag_tokens
        },
    }
}

fn generate_opcodes(
    writer: &mut BufWriter<File>,
    unprefixed: &Vec<InputOpcode>,
    cbprefixed: &Vec<InputOpcode>,
) -> Result<(), Error> {
    let unprefixed_array_tokens = generate_opcodes_array(unprefixed, "UNPREFIXED_OPCODES");
    let cbprefixed_array_tokens = generate_opcodes_array(cbprefixed, "CBPREFIXED_OPCODES");

    let generated_code = quote! {
        use lazy_static::lazy_static;

        struct Operand {
            pub name: &'static str,
            pub immediate: bool,
            pub bytes: u8,
        }

        struct Flag {
            pub z: &'static str,
            pub n: &'static str,
            pub h: &'static str,
            pub c: &'static str,
        }

        struct Opcode {
            name: &'static str,
            mnemonic: &'static str,
            bytes: u8,
            cycles: Vec<u8>,
            operands: Vec<Operand>,
            immediate: bool,
            flags: Flag,
        }

        lazy_static! {
            #unprefixed_array_tokens

            #cbprefixed_array_tokens
        }
    };

    writer.write_all(generated_code.to_string().as_bytes())?;

    // It's important to flush the BufWriter to ensure all buffered data is written to the file
    writer.flush()?;

    Ok(())
}

fn generate_opcodes_array(input_opcodes: &Vec<InputOpcode>, symbol_name: &str) -> TokenStream {
    let num_opcodes = input_opcodes.len();

    let symbol_ident = format_ident!("{}", symbol_name);

    let mut opcodes_token_stream: TokenStream = TokenStream::new();
    for input_opcode in input_opcodes.iter() {
        opcodes_token_stream.append_all(generate_opcode(input_opcode));
    }

    quote! {

            static ref #symbol_ident : [Opcode; #num_opcodes ] = [
                #opcodes_token_stream
            ];
    }
}

/// Given a rust file, parse it and make it look pretty
fn format_file(input_filename: &str, output_filename: &str) -> Result<(), Error> {
    let content = fs::read_to_string(input_filename)?;
    let syntax_tree = syn::parse_file(content.as_str())
        .expect(format!("Couldn't parse {:?}", input_filename).as_str());
    
    let formatted = prettyplease::unparse(&syntax_tree);
    let mut formatted_file: File = File::create(output_filename)?;
    formatted_file.write_all(formatted.as_bytes())?;

    event!(Level::INFO, "Created {}", output_filename);
    Ok(())
}


#[instrument]

/// Uses the Opcodes.json file in the current directory to
/// generate two arrays:
///   - Unprefixed opcodes
///   - CBPrefixed opcodes
///
/// Writes the files out to ../src/gen_opcodes.rs
pub fn generate() -> Result<(), Error> {
    // Open the file
    let opcodes_json_file = File::open("Opcodes.json")?;
    let reader = BufReader::new(opcodes_json_file);

    // Parse the JSON
    let v: Value = serde_json::from_reader(reader).unwrap();

    // Print the parsed data
    event!(
        Level::INFO,
        "Got first value as: {:?}\n",
        v["unprefixed"]["0x00"]
    );
    let unprefixed_values = v["unprefixed"].as_object().unwrap();
    let cbprefixed_values = v["cbprefixed"].as_object().unwrap();

    let unprefixed = deserialize_opcodes(unprefixed_values)?;
    let cbprefixed = deserialize_opcodes(cbprefixed_values)?;

    {
        let gen_file = File::create(TEMP_FILENAME)?;
        let mut writer = BufWriter::new(gen_file);
        generate_opcodes(&mut writer, &unprefixed, &cbprefixed)?;
        writer.flush()?;
    }

    format_file(TEMP_FILENAME, GEN_FILENAME)?;

    Ok(())
}