use pest::Parser;
use pest::iterators::Pair;
use std::fs::read_to_string;

#[derive(Parser)]
#[grammar = "stardict_grammar.pest"]
pub struct StardictParser;

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Sentence {
    pub viet: String,
    pub eng: String,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Sense {
    pub part_of_speech: String,
    pub definition: String,
    pub sentences: Vec<Sentence>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Entry {
    pub word: String,
    pub senses: Vec<Sense>,
}

#[derive(PartialEq, Eq, Debug, Clone, Default)]
pub struct Dictionary {
    pub entries: Vec<Entry>
}

fn parse_sentence(p: Pair<Rule>) -> Sentence {
    let mut sentence: Sentence = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::viet => sentence.viet = field
                .as_str()
                .trim()
                .to_string(),
            Rule::eng => sentence.eng = field
                .as_str()
                .trim()
                .to_string(),
            _ => println!("Error, fields not found"),
        }
    }
    sentence
}

fn parse_def(p: Pair<Rule>) -> Sense {
    let mut definition: Sense = Default::default();
    for field in p.into_inner() {
        match field.as_rule() {
            Rule::part_of_speech => definition.part_of_speech = field
                .as_str()
                .trim()
                .to_string(),
            Rule::definition => definition.definition = field
                .as_str()
                .trim()
                .to_string(),
            Rule::sentence => definition.sentences.push(parse_sentence(field)),
            _ => println!("Error, fields not found"),
        }
    }
    definition
}

pub fn parse_entry(raw: &str) -> Entry {
    let data = StardictParser::parse(Rule::entry, raw)
        .expect("cannot parse")
        .next()
        .unwrap();
    let mut entry: Entry = Default::default();
    for field in data.into_inner() {
        match field.as_rule() {
            Rule::word => entry.word = field
                .as_str()
                .trim()
                .to_string(),
            Rule::sense => entry.senses.push(parse_def(field)),
            _ => println!("Error, fields not found"),
        }
    }
    entry
}

#[test]
fn test_parse_entry_basic() {
    let raw = 
        "@an phận\n\
        * verb\n\
        - To feel smug\n\
        =tư tưởng an phận+Smugness, smug feeling\n\
        =an phận thủ thường+to feel smug about one's present circumstances \n\n";

    let entry = Entry {
        word: "an phận".to_string(),
        senses: vec![
            Sense {
                part_of_speech: "verb".to_string(),
                definition: "To feel smug".to_string(),
                sentences: vec![
                    Sentence { 
                        viet: "tư tưởng an phận".to_string(), 
                        eng: "Smugness, smug feeling".to_string(),
                    },
                    Sentence { 
                        viet: "an phận thủ thường".to_string(), 
                        eng: "to feel smug about one's present circumstances".to_string(),
                    },
                ],
            },
        ],
    };

    assert_eq!(parse_entry(raw), entry);
}
