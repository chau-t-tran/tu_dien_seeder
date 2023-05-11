use crate::dictionary::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};

pub struct EntryIterator {
    reader: BufReader<File>,
}

impl Iterator for EntryIterator {
    type Item = Entry;
    fn next(&mut self) -> Option<Self::Item> {
        let mut lines = Vec::new();
        loop {
            let mut buffer = String::new();
            match self.reader.read_line(&mut buffer) {
                Ok(0) => return None, // end of line
                Ok(_) => {
                    let line = buffer.trim().to_string();
                    lines.push(line.to_string());
                    if line.is_empty() {
                        break;
                    }
                }
                Err(_) => return None,
            }
        }
        let raw = lines.join("\n");
        let entry = parse_entry(raw.as_str());
        Some(entry)
    }
}

impl EntryIterator {
    pub fn new(file: File) -> EntryIterator {
        EntryIterator {
            reader: BufReader::new(file),
        }
    }
}

#[test]
fn test_parse_entry_by_entry_from_file() {
    use pretty_assertions::{assert_eq, assert_ne};

    let file = File::open("src/viet-eng.txt").expect("Could not open file");
    let mut entry_iterator = EntryIterator::new(file);

    let first_entry: Entry = entry_iterator.next().unwrap();
    let first_expected = Entry {
        word: "a".to_string(),
        senses: vec![
            Sense {
                part_of_speech: "excl".to_string(),
                definition: "O; oh (exclamation of surprise, regret, ect.)".to_string(),
                sentences: vec![
                    Sentence {
                        eng: "Oh! What a nice toy!".to_string(),
                        viet: "a, đồ chơi đẹp quá!".to_string(),
                    },
                    Sentence {
                        eng: "Oh! What a pity!".to_string(),
                        viet: "a, tội nghiệp quá".to_string(),
                    },
                ],
            },
            Sense {
                part_of_speech: "".to_string(),
                definition: "By the way".to_string(),
                sentences: vec![Sentence {
                    eng: "By the way, there's this one other question".to_string(),
                    viet: "a, còn một vấn đề này nữa".to_string(),
                }],
            },
            Sense {
                part_of_speech: "noun".to_string(),
                definition: "Acre (100 square meters)".to_string(),
                sentences: vec![],
            },
        ],
    };

    let second_entry: Entry = entry_iterator.next().unwrap();
    let second_expected = Entry {
        word: "a dua".to_string(),
        senses: vec![Sense {
            part_of_speech: "verb".to_string(),
            definition: "To ape, to chime in, to join in, to take a leaf out of sb's book"
                .to_string(),
            sentences: vec![
                Sentence {
                    viet: "a dua theo lối ăn mặc lố lăng".to_string(),
                    eng: "to ape other's eccentric style of dress".to_string(),
                },
                Sentence {
                    viet: "người hay a dua bắt chước".to_string(),
                    eng: "a copy-cat".to_string(),
                },
            ],
        }],
    };

    let third_entry: Entry = entry_iterator.next().unwrap();
    let third_expected = Entry {
        word: "a ha".to_string(),
        senses: vec![Sense {
            part_of_speech: "excl".to_string(),
            definition: "Aha, ha; hurrah, hurray".to_string(),
            sentences: vec![
                Sentence {
                    viet: "a ha! tên trộm bị cảnh sát tóm rồi!".to_string(),
                    eng: "Ha! the thief is caught by the police!".to_string(),
                },
                Sentence {
                    viet: "a ha! cô gái xinh quá!".to_string(),
                    eng: "hurrah! What a pretty girl!".to_string(),
                },
            ],
        }],
    };

    assert_eq!(first_entry, first_expected);
    assert_eq!(second_entry, second_expected);
    assert_eq!(third_entry, third_expected);
}

#[test]
fn test_parse_until_end() {
    use pretty_assertions::{assert_eq, assert_ne};

    let file = File::open("src/viet-eng.txt").expect("Could not open file");
    let mut entry_iterator = EntryIterator::new(file);

    for val in entry_iterator {
        println!("{:?}", val)
    }
}
