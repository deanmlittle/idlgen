use std::fs::File;
use std::io::{BufReader, Write};
use std::str::FromStr;
use anyhow::Result;
use clap::Parser;
use convert_case::{Case, Casing};
use generators::common::make_sdk;
use types::IDL;
mod types;
mod generators;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filename of IDL file
    #[arg(short, long)]
    filename: String,
}

fn make(idl: &IDL) -> Result<()> {
    let mut file = File::create(format!("{}.rs", idl.name.to_case(Case::Snake)))?;
    file.write_all(make_sdk(idl).as_bytes())?;
    Ok(())
}

fn main() -> Result<()> { 
    let args = Args::parse();
    let file = File::open(&args.filename)?;
    let reader = BufReader::new(file);

    let idl: IDL = serde_json::from_reader(reader)?;

    make(&idl)?;

    Ok(())
}
