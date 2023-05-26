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
mod hydrator;
mod sql;
mod unhydrated_iterator;
mod word_map;

use crate::dictionary::*;
use crate::entry_iterator::*;
use crate::hydrator::*;
use crate::sql::*;
use crate::word_map::*;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = Command::new("stardict")
        .arg(
            Arg::new("command")
                .required(true)
                .value_parser(["parse", "hydrate"]),
        )
        .arg(
            Arg::new("filepath")
                .required_if_eq_any([("command", "parse"), ("command", "hydrate")])
                .long("filepath"),
        )
        .get_matches();

    let value = match matches.get_one::<String>("command").unwrap().as_str() {
        "parse" => {
            let filepath = matches.get_one::<String>("filepath").unwrap().as_str();
            parse(filepath)
        }
        "hydrate" => {
            let filepath = matches.get_one::<String>("filepath").unwrap();
            let word_map = WordMap::new(filepath.to_string());
            let run_iterate = iterate_unhydrated();
            let run_server = run();
            futures::future::join(run_server, run_iterate).await;
            Ok(())
        }
        _ => unreachable!(),
    };

    value
}
