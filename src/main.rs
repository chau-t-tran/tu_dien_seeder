#![allow(unused)]
use clap::{Arg, Args, Command};
use core::time::Duration;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod dictionary;
mod entry_iterator;
mod fpt_client;
mod hydrator;
mod sql;
mod unhydrated_iterator;

use crate::dictionary::*;
use crate::entry_iterator::*;
use crate::hydrator::*;
use crate::sql::*;
use crate::unhydrated_iterator::*;

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
            let word_map = Arc::new(Mutex::new(HashMap::new()));
            let run_iterate = iterate_unhydrated(word_map.clone());
            let run_server = run(word_map.clone());
            futures::future::join(run_server, run_iterate).await;
            Ok(())
        }
        _ => unreachable!(),
    };

    value
}

pub async fn iterate_unhydrated(word_map: Arc<Mutex<HashMap<String, String>>>) {
    let mut conn = init_db().unwrap();
    let untranslated = UnhydratedIterator::new(&mut conn);
    let duration = Duration::new(1, 500);

    println!("Iterating through words...");
    for text in untranslated {
        println!("{}", text);
        sleep(duration).await;
    }
}
