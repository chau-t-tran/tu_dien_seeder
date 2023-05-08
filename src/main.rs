#![allow(unused)]
use clap::{Command, Args, Arg};
use std::path::PathBuf;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

use std::result::Result;
use std::error::Error;

extern crate pest;
#[macro_use]
extern crate pest_derive; 

mod dictionary;
mod iterators {
    pub mod entry_iterator;
    pub mod lexico_iterator;
}
mod sql;

use crate::dictionary::*;
use crate::iterators::entry_iterator::*;
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
        insert_entry(entry, &mut pool);
        stdout.flush()?;
    }
    print!("\nFinished!");

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
            println!("Hydrating...");
            Ok(())
        }
        _ => unreachable!(),
    };

    value
}
