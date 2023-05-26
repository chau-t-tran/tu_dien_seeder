use crate::dictionary::*;
use crate::sql::*;
use mysql::prelude::*;
use mysql::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};

pub struct UnhydratedIterator<'a> {
    conn: &'a mut PooledConn,
    current_id: i32,
}

impl<'a> Iterator for UnhydratedIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        match query_next_lexico(self.conn, self.current_id) {
            Ok(word) => {
                self.current_id += 1;
                Some(word)
            }
            Err(error) => match error {
                LexicoError::UnknownError() => None,
                LexicoError::SqlError(error) => {
                    eprint!("{}", error);
                    None
                }
            },
        }
    }
}

impl UnhydratedIterator<'_> {
    pub fn new(conn: &mut PooledConn) -> UnhydratedIterator {
        UnhydratedIterator {
            conn,
            current_id: 1,
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

fn query_next_lexico(conn: &mut PooledConn, id: i32) -> std::result::Result<String, LexicoError> {
    let query_result: Row = conn
        .exec_first(GET_FIRST, (id,))?
        .ok_or(LexicoError::UnknownError())?;
    let row_value = query_result.as_ref(0).ok_or(LexicoError::UnknownError())?;
    match row_value {
        Value::Bytes(value) => Ok(String::from_utf8_lossy(value.as_slice()).into_owned()),
        _ => Err(LexicoError::UnknownError()),
    }
}

const GET_FIRST: &str = r"
SELECT text
FROM text_audio
WHERE (audio_url IS NULL OR audio_url = '') AND id = (?)
;";

#[test]
fn test_query_next_lexico() {
    let mut conn = init_db().expect("Error connecting to DB");
    let word = query_next_lexico(&mut conn, 1).unwrap();
    assert_eq!(word, "a");
}
