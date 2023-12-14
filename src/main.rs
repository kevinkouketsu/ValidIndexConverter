pub mod binary_data;
pub mod json_data;
pub mod text_data;

use binary_data::BinaryData;
use clap::{Parser, ValueEnum};
use deku::prelude::*;
use json_data::JsonData;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use text_data::TextData;

const MAX_VALID_ANI_LIST: usize = 100;
const SIZE_OF_VALID_ANI_LIST: usize = 186;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
struct Animation {
    #[deku(count = "SIZE_OF_VALID_ANI_LIST")]
    index: Vec<u32>,
}
impl Default for Animation {
    fn default() -> Self {
        Self {
            index: vec![0; SIZE_OF_VALID_ANI_LIST],
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, DekuRead, DekuWrite)]
pub struct Bone {
    #[deku(count = "MAX_VALID_ANI_LIST")]
    ani: Vec<Animation>,
}
impl Default for Bone {
    fn default() -> Self {
        Self {
            ani: vec![Animation::default(); MAX_VALID_ANI_LIST],
        }
    }
}

pub trait Format<T> {
    fn read<R: Read>(reader: &mut R) -> Result<T, std::io::Error>;
    fn write<W: Write>(writer: &mut W, ani_list: Bone) -> Result<(), std::io::Error>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Operation {
    Json,
    Bin,
    Text,
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(long = "io", value_enum)]
    input_operation: Operation,

    #[arg(long = "oo", value_enum)]
    output_operation: Operation,

    #[arg(short = 'i', long = "input")]
    input: String,

    #[arg(short = 'o', long = "output")]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    let mut input_file = std::fs::File::open(cli.input).unwrap();
    let list = match cli.input_operation {
        Operation::Json => JsonData::read(&mut input_file),
        Operation::Bin => BinaryData::read(&mut input_file),
        Operation::Text => TextData::read(&mut input_file),
    }
    .expect("Coult not read input file");

    let mut output_file = std::fs::File::create(cli.output).unwrap();
    match cli.output_operation {
        Operation::Json => JsonData::write(&mut output_file, list),
        Operation::Bin => BinaryData::write(&mut output_file, list),
        Operation::Text => TextData::write(&mut output_file, list),
    }
    .expect("Could not save output file");
}
