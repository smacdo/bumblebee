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
const PANGRAM_SCORE_BOOST: i32 = 7;
const SCORE_MIN_LENGTH: usize = 5;
const WORD_MIN_LENGTH: usize = 4;

#[derive(Debug, PartialEq)]
pub struct Answer {
    pub word: String,
    pub score: i32,
    pub is_pangram: bool,
}

/// Find all valid answers given an iterable list of potential words.
pub fn find_all<I, S>(words: I, required: char, extra: &str) -> Vec<Answer>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    // IntoIterator inspiration from: https://stackoverflow.com/a/35626785
    let mut answers: Vec<Answer> = Vec::new();

    for w in words {
        match check_word(w.as_ref(), required, extra) {
            Some(ans) => answers.push(ans),
            None => {}
        };
    }

    answers
}

/// Test if the given word `word` is a valid answer to the problem. A word is
/// considered a solution if it is at least four letters long, at least one
/// character matches `required`, and the remaining letters match either
/// `required` or one of the values in `extra`.
pub fn check_word(word: &str, required: char, extra: &str) -> Option<Answer> {
    // Words must be at least four characters.
    if word.len() < WORD_MIN_LENGTH {
        return None;
    }

    // Words must also contain the required character.
    if !word.contains(required) {
        return None;
    }

    // Words can only contain characters matching required or extra.
    if word
        .chars()
        .into_iter()
        .all(|x| x == required || extra.chars().any(|e| e == x))
    {
        // Count the number of unique letters that were matched. We do this with
        // a O(nm) algorithm to avoid allocating a hashmap since both n and m
        // are small.
        let mut uniq_count = 1; // The required char must always match.

        for e in extra.chars() {
            if word.chars().any(|w| w == e) {
                uniq_count += 1;
            }
        }

        let is_pangram = uniq_count == 1 + extra.len();

        // Scoring uses the following rules:
        //  1. Four letter words score 1 point.
        //  2. Five letter or longer words score their length in points.
        //  3. A pangram receives an extra 7 points.
        let mut score: i32 = 1;

        if word.len() >= SCORE_MIN_LENGTH {
            score = word.len() as i32;
        }

        if is_pangram {
            score += PANGRAM_SCORE_BOOST;
        }

        // Return answer as the word, its score and if it was a pangram.
        Some(Answer {
            word: word.to_string(),
            score,
            is_pangram,
        })
    } else {
        None
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::{check_word, find_all};

    #[test]
    fn empty_word_is_not_valid() {
        assert_eq!(None, check_word("", 'c', "ab"));
    }

    #[test]
    fn word_less_than_four_chars_not_valid() {
        assert_eq!(None, check_word("c", 'c', "ab"));
        assert_eq!(None, check_word("ca", 'c', "ab"));
        assert_eq!(None, check_word("cab", 'c', "ab"));

        assert_eq!(None, check_word("d", 'c', "ab"));
        assert_eq!(None, check_word("do", 'c', "ab"));
        assert_eq!(None, check_word("dog", 'c', "ab"));
    }

    #[test]
    fn word_uses_required_valid() {
        assert!(check_word("tttt", 't', "oe").is_some());
        assert!(check_word("tttttt", 't', "oe").is_some());
        assert!(check_word("ssss", 's', "oe").is_some());
    }

    #[test]
    fn word_missing_required_not_valid() {
        assert_eq!(None, check_word("oeoe", 't', "oe"));
        assert_eq!(None, check_word("oooo", 't', "oe"));
    }

    #[test]
    fn word_can_use_extra_chars() {
        assert!(check_word("tote", 't', "elom").is_some());
        assert!(check_word("totet", 't', "elom").is_some());
        assert!(check_word("mote", 't', "elom").is_some());
        assert!(check_word("molet", 't', "elom").is_some());
    }

    #[test]
    fn word_with_chars_not_in_required_or_extra_not_valid() {
        assert_eq!(None, check_word("dote", 't', "elom"));
        assert_eq!(None, check_word("note", 't', "elom"));
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
        assert_eq!("tote", answers[0].word);
        assert_eq!("mote", answers[1].word);
    }

    #[test]
    fn answer_contains_original_word() {
        assert_eq!("motel", check_word("motel", 't', "elom").unwrap().word);
        assert_eq!("tome", check_word("tome", 't', "elom").unwrap().word);
    }

    #[test]
    fn pangram_uses_all_letters() {
        assert_eq!(true, check_word("motel", 't', "elom").unwrap().is_pangram);
        assert_eq!(true, check_word("emotel", 't', "elom").unwrap().is_pangram);
        assert_eq!(false, check_word("motee", 't', "elom").unwrap().is_pangram);
        assert_eq!(false, check_word("mote", 't', "elom").unwrap().is_pangram);
    }

    #[test]
    fn four_length_score_is_one() {
        assert_eq!(1, check_word("tome", 't', "elom").unwrap().score);
        assert_eq!(1, check_word("tell", 't', "elom").unwrap().score);
    }

    #[test]
    fn score_equal_length_when_larger_than_four() {
        assert_eq!(5, check_word("motee", 't', "elom").unwrap().score);
        assert_eq!(5, check_word("tello", 't', "elom").unwrap().score);
        assert_eq!(6, check_word("tomtom", 't', "elom").unwrap().score);
        assert_eq!(9, check_word("tomtomtom", 't', "elom").unwrap().score);
    }

    #[test]
    fn pangram_adds_seven_to_score() {
        assert_eq!(12, check_word("motel", 't', "elom").unwrap().score);
        assert_eq!(13, check_word("emotel", 't', "elom").unwrap().score);
    }
}
