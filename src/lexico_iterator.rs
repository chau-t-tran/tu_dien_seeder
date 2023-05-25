use crate::dictionary::*;
use crate::sql::*;
use mysql::prelude::*;
use mysql::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};

pub struct LexicoIterator<'a> {
    conn: &'a mut PooledConn,
    current_word: Option<String>,
}

impl<'a> Iterator for LexicoIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match &self.current_word {
            Some(w) => match query_next_lexico(self.conn, w.to_string()) {
                Ok(new_word) => {
                    self.current_word = Some(new_word);
                    Some(self.current_word.as_ref().unwrap().clone())
                }
                Err(error) => match error {
                    LexicoError::UnknownError() => {
                        eprint!("Some unknown query error occurred!");
                        None
                    }
                    LexicoError::SqlError(error) => {
                        eprint!("{}", error);
                        None
                    }
                },
            },
            _ => None,
        }
    }
}

impl LexicoIterator<'_> {
    pub fn new(conn: &mut PooledConn) -> LexicoIterator {
        LexicoIterator {
            conn,
            current_word: Some(String::from("")),
        }
    }
}

#[derive(Debug)]
enum LexicoError {
    SqlError(mysql::Error),
    UnknownError(),
}

impl From<mysql::Error> for LexicoError {
    fn from(error: mysql::Error) -> Self {
        LexicoError::SqlError(error)
    }
}

fn query_next_lexico(
    conn: &mut PooledConn,
    word: String,
) -> std::result::Result<String, LexicoError> {
    let query_result: Row = conn
        .exec_first(GET_FIRST, (word,))?
        .ok_or(LexicoError::UnknownError())?;
    let row_value = query_result.as_ref(0).ok_or(LexicoError::UnknownError())?;
    match row_value {
        Value::Bytes(value) => Ok(String::from_utf8_lossy(value.as_slice()).into_owned()),
        _ => Err(LexicoError::UnknownError()),
    }
}

const GET_FIRST: &str = r"
SELECT DISTINCT text, MIN(id) as id
FROM text_audio 
WHERE text > (?)
GROUP BY text
ORDER BY text ASC
LIMIT 1;";

#[test]
fn test_query_next_lexico() {
    let mut conn = init_db().expect("Error connecting to DB");
    let word = query_next_lexico(&mut conn, String::from("")).unwrap();
    assert_eq!(word, "a");
}
