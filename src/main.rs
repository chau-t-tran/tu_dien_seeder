#![allow(unused)]
use clap::{Arg, Args, Command};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;

use std::error::Error;
use std::result::Result;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod dictionary;
mod entry_iterator;
mod file_manager;
mod hydrator;
mod lexico_iterator;
mod sql;

use crate::dictionary::*;
use crate::entry_iterator::*;
use crate::hydrator::*;
use crate::sql::*;

fn parse(filepath: &str) -> Result<(), Box<dyn Error>> {
    let file = File::open(filepath)?;

    let mut pool = init_db()?;
    let mut dict = EntryIterator::new(file);
    let mut stdout = io::stdout().lock();

    for (i, entry) in dict.enumerate() {
        let line = format!("\r Word #{}: {}", i, entry.word.clone());
        print!("{}", " ".repeat(line.len()));
        print!("{}", line);
        match insert_entry(entry, &mut pool) {
            Ok(_) => (),
            Err(val) => {
                println!("Error! {}", val);
                break;
            }
        };
        stdout.flush()?;
    }
    println!("\nFinished!");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("stardict")
        .arg(
            Arg::new("command")
                .required(true)
                .value_parser(["parse", "hydrate"]),
        )
        .arg(
            Arg::new("filepath")
                .required_if_eq("command", "parse")
                .long("filepath"),
        )
        .get_matches();

    let value = match matches.get_one::<String>("command").unwrap().as_str() {
        "parse" => {
            let filepath = matches.get_one::<String>("filepath").unwrap().as_str();
            parse(filepath)
        }
        "hydrate" => {
            run();
            Ok(())
        }
        _ => unreachable!(),
    };

    value
}
