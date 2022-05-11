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
use spellingbee::{find_all, Answer};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::{fs::File, path::Path};

const APP_SHORT_NAME: &str = "spellingbee";

/// Command line parameters.
#[derive(Parser)]
#[clap(name = "Spellingbee")]
#[clap(author = "Scott MacDonald <scott@smacdo.com>")]
#[clap(about = "Finds answers to the spelling bee game.")]
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
    let answers = find_all_with_dict(args.dict_path, args.required_char, &args.extra_chars);

    match answers {
        Ok(mut answers) => {
            // Print pangrams answers before all other answers, but make sure
            // always show answers in order of descending score.
            answers.sort_unstable_by_key(|a| -a.score);

            for ans in answers.iter().filter(|&a| a.is_pangram) {
                println!("* {:<2} {}", ans.score, ans.word);
            }

            for ans in answers.iter().filter(|&a| !a.is_pangram) {
                println!("  {:<2} {}", ans.score, ans.word);
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
) -> std::io::Result<Vec<Answer>> {
    let raw_file = File::open(path)?;
    let file = BufReader::new(raw_file);

    Ok(find_all(
        file.lines()
            .map(|maybe_line| maybe_line.expect("Failed to read line from dictionary")),
        required,
        extra,
    ))
}
