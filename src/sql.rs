use crate::dictionary::*;
use crate::entry_iterator::*;
use config::{Config, File, FileFormat};
use mysql::prelude::*;
use mysql::*;
use std::error::Error;
use std::result::Result;

pub fn init_db() -> Result<PooledConn, Box<dyn Error>> {
    let settings = Config::builder()
        .add_source(File::new("config/MySQL.toml", FileFormat::Toml))
        .build()?;

    let mysql_url = settings.get_string("mysql_url")?;
    let pool = Pool::new(mysql_url.as_str())?;
    let mut conn = pool.get_conn()?;

    Ok(conn)
}

pub fn insert_text(text: String, conn: &mut PooledConn) -> Result<(), Box<dyn Error>> {
    conn.exec_drop("INSERT IGNORE INTO text_audio (text) VALUES (?)", (text,))?;
    Ok(())
}

pub fn insert_entry(entry: Entry, conn: &mut PooledConn) -> Result<(), Box<dyn Error>> {
    for sense in entry.senses {
        conn.exec_drop(
            "INSERT INTO entries (word, pos, def) VALUES (?, ?, ?)",
            (
                entry.word.clone(),
                sense.part_of_speech.clone(),
                sense.definition.clone(),
            ),
        )?;
        let entry_id = conn.last_insert_id();
        insert_text(entry.word.clone(), conn);
        for sentence in sense.sentences {
            conn.exec_drop(
                "INSERT INTO sentences (eng, viet, entry_id) VALUES (?, ?, ?)",
                (sentence.eng, sentence.viet.clone(), entry_id),
            )?;
            insert_text(sentence.viet, conn);
        }
    }
    Ok(())
}

#[ignore]
#[test]
fn test_insert_db() -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open("src/viet-eng.txt")?;
    let mut entry_iterator = EntryIterator::new(file);

    let mut conn = init_db()?;

    match entry_iterator.next() {
        Some(entry) => {
            insert_entry(entry, &mut conn).expect("Error inserting entry");
            conn.exec_drop("DELETE FROM sentences", ())
                .expect("Error resetting table, delete manually");
            conn.exec_drop("DELETE FROM entries", ())
                .expect("Error resetting table, delete manually");
        }
        _ => panic!("No value found from iterator"),
    }
    Ok(())
}

#[test]
fn test_init_db() -> Result<(), Box<dyn Error>> {
    let conn = init_db()?;
    assert!(true);
    Ok(())
}
