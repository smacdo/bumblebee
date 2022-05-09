////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Scott MacDonald.
////////////////////////////////////////////////////////////////////////////////
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
////////////////////////////////////////////////////////////////////////////////
// TODO: Windows support since it doesn't have a builtin dictionary?
use clap::Parser;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fs::File, path::Path};

const APP_SHORT_NAME: &str = "bumblebee";

/// Command line parameters.
#[derive(Parser)]
#[clap(name = "Bumblebee")]
#[clap(author = "Scott MacDonald <scott@smacdo.com>")]
#[clap(about = "Finds answers to the NYT spelling bee game", long_about = None)]
struct CliParams {
    /// Path to a dictionary file (one word per line).
    #[clap(short = 'd')]
    #[clap(default_value = "/usr/share/dict/words")]
    dict_path: PathBuf,
    /// Character required to be in every answer.
    required_char: char,
    /// Extra characters allowed to be in an answer.
    extra_chars: String,
}

/// Application entry point.
fn main() {
    let args = CliParams::parse();

    // Print the matching words or print any errors encountered when trying to
    // load the dictionary.
    let words = find_all_with_dict(args.dict_path, args.required_char, &args.extra_chars);

    match words {
        Ok(words) => {
            for w in words {
                println!("{}", &w);
            }
        }
        Err(err) => {
            eprintln!(
                "{} error: Failed to load dictionary ({:?})",
                APP_SHORT_NAME, err
            );
        }
    };
}

/// Find all valid answers given a path to a dictionary file specified by `path`.
/// It is expected that the dictionary file contains one word per line.
fn find_all_with_dict<P: AsRef<Path>>(
    path: P,
    required: char,
    extra: &str,
) -> std::io::Result<Vec<String>> {
    let raw_file = File::open(path)?;
    let file = BufReader::new(raw_file);

    Ok(find_all(
        file.lines()
            .map(|maybe_line| maybe_line.expect("Failed to read line from dictionary")),
        required,
        extra,
    ))
}

/// Find all valid answers given an iterable list of potential words.
/// IntoIterator fix: https://stackoverflow.com/a/35626785
fn find_all<I, S>(words: I, required: char, extra: &str) -> Vec<String>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut answers: Vec<String> = Vec::new();

    for w in words {
        if is_valid_word(w.as_ref(), required, extra) {
            answers.push(w.as_ref().to_string());
        }
    }

    answers
}

/// Test if the given word `word` is a valid answer to the problem. A word is
/// considered a solution if it is at least four letters long, at least one
/// character matches `required`, and the remaining letters match either
/// `required` or one of the values in `extra`.
pub fn is_valid_word(word: &str, required: char, extra: &str) -> bool {
    // Words must be at least four characters.
    if word.len() < 4 {
        return false;
    }

    // Words must also contain the required character.
    if !word.contains(required) {
        return false;
    }

    // Words can only contain characters matching required or extra.
    word.chars()
        .into_iter()
        .all(|x| x == required || extra.chars().any(|e| e == x))
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{find_all, is_valid_word};

    #[test]
    fn empty_word_is_not_valid() {
        assert_eq!(false, is_valid_word("", 'c', "ab"));
    }

    #[test]
    fn word_less_than_four_chars_not_valid() {
        assert_eq!(false, is_valid_word("c", 'c', "ab"));
        assert_eq!(false, is_valid_word("ca", 'c', "ab"));
        assert_eq!(false, is_valid_word("cab", 'c', "ab"));
    }

    #[test]
    fn word_uses_required_valid() {
        assert_eq!(true, is_valid_word("tttt", 't', "oe"));
        assert_eq!(true, is_valid_word("tttttt", 't', "oe"));
        assert_eq!(true, is_valid_word("ssss", 's', "oe"));
    }

    #[test]
    fn word_missing_required_not_valid() {
        assert_eq!(false, is_valid_word("oeoe", 't', "oe"));
        assert_eq!(false, is_valid_word("oooo", 't', "oe"));
    }

    #[test]
    fn word_can_use_extra_chars() {
        assert_eq!(true, is_valid_word("tote", 't', "elom"));
        assert_eq!(true, is_valid_word("totet", 't', "elom"));
        assert_eq!(true, is_valid_word("mote", 't', "elom"));
        assert_eq!(true, is_valid_word("molet", 't', "elom"));
    }

    #[test]
    fn word_with_chars_not_in_required_or_extra_not_valid() {
        assert_eq!(false, is_valid_word("dote", 't', "elom"));
        assert_eq!(false, is_valid_word("note", 't', "elom"));
    }

    #[test]
    fn test_multiple_words() {
        let words = vec![
            "tote".to_string(),
            "vote".to_string(),
            "mote".to_string(),
            "soapy".to_string(),
        ];
        let answers = find_all(words.iter(), 't', "elom");
        assert_eq!(2, answers.len());
        assert_eq!("tote", answers[0]);
        assert_eq!("mote", answers[1]);
    }
}
