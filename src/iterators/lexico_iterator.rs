use crate::dictionary::*;
use crate::sql::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};
use std::error::Error;
use mysql::*;
use mysql::prelude::*;

pub struct LexicoIterator<'a> {
    conn: &'a mut PooledConn,
    current_word: Option<String>,
}

impl<'a> Iterator for LexicoIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.current_word {
            Some(w) => {
                let word = query_next_lexico(self.conn, w.to_string()).ok()?;
                self.current_word = word;
                Some(self.current_word.as_ref()
                                      .unwrap()
                                      .clone())
            }
            _ => None
        }
    }
}

impl LexicoIterator<'_> {
    pub fn new(conn: &mut PooledConn) -> LexicoIterator {
        LexicoIterator {
            conn: conn,
            current_word: Some(String::from("")),
        }
    }
}

fn query_next_lexico(conn: &mut PooledConn, word: String) -> Result<Option<String>> {
    let row: Row = conn.exec_first(GET_FIRST, (word,))?.unwrap();
    if let Some(row) = row.as_ref(0) {
        let value = match row {
            Value::Bytes(v) => Some(String::from_utf8_lossy(v.as_slice()).into_owned()),
            _  => None
        };
        return Ok(value);
    }
    Ok(Some(String::from("")))
}

const GET_FIRST: &str = r"
SELECT DISTINCT word, MIN(id) as id
FROM entries
WHERE word > (?)
GROUP BY word
ORDER BY word ASC
LIMIT 1;";

#[test]
fn test_query_next_lexico() {
    let mut conn = init_db().expect("Error connecting to DB");
    let word = query_next_lexico(&mut conn, String::from("")).expect("Error grabbing word")
                                                             .unwrap();
    assert_eq!(word, "a");
}
