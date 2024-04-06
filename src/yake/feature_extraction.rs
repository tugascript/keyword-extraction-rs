// Copyright (C) 2024 Afonso Barracha
//
// Rust Keyword Extraction is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rust Keyword Extraction is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Rust Keyword Extraction. If not, see <http://www.gnu.org/licenses/>.

use std::collections::{HashMap, HashSet};

use unicode_segmentation::UnicodeSegmentation;

use crate::common::{get_capitalized_regex, get_upper_case_regex};

use super::context_builder::{ContextBuilder, WordContext};

/**
 * Formula:
 *  H = (WPos * WRel) / (WCas + (WFreq/WRel) + (WDif/WRel))
**/
pub struct FeaturedWord {
    // Casing
    cas: f32, // Done
    // Frequency
    tf: f32, // Done
    // Positional
    pos: f32, // Done
    // Relatedness
    rel: f32, // Done
    // Different sentence
    dif: f32, // Done
}

type TfCaps = f32;
type TfUpper = f32;
type TfAll = f32;
type TfCasing = (TfCaps, TfUpper, TfAll);
type CasingMap = HashMap<String, TfCasing>;

pub struct FeatureExtraction<'a> {
    words: Vec<&'a str>,
    sentences: Vec<Vec<&'a str>>,
    context: WordContext<'a>,
}

fn generate_casing_map<'a>(words: &'a [&'a str]) -> CasingMap {
    words.iter().fold(HashMap::new(), |mut cm, w| {
        let value = cm.entry(w.to_lowercase()).or_insert((0.0, 0.0, 0.0));
        value.2 += 1.0;

        if w.graphemes(true).count() == 1 {
            return cm;
        }
        match get_upper_case_regex() {
            Some(regex) => {
                if regex.is_match(w) {
                    value.0 += 1.0;
                }
            }
            None => {
                if w.to_uppercase().as_str() == *w {
                    value.0 += 1.0;
                }
            }
        };
        match get_capitalized_regex() {
            Some(regex) => {
                if regex.is_match(w) {
                    value.1 += 1.0;
                }
            }
            None => {
                if w.chars().next().unwrap().is_uppercase()
                    && w.chars().skip(1).all(char::is_lowercase)
                {
                    value.1 += 1.0;
                }
            }
        };
        cm
    })
}

/**
 * Formula:
 * WCase = MAX(TfCaps, TfUpper) / (1 + ln(TfAll))
**/
fn calculate_casing(casing_map: CasingMap) -> HashMap<String, f32> {
    casing_map
        .into_iter()
        .map(|(word, (caps, upper, all))| {
            let max = if caps > upper { caps } else { upper };
            (word, max / (1.0 + all.ln()))
        })
        .collect()
}

/**
 * Formula:
 * WFreq = TfAll / (avgTf + stdTf)
**/
fn calculate_tf(casing_map: CasingMap) -> HashMap<String, f32> {
    let count = casing_map.len() as f32;
    let avg = casing_map.values().fold(0.0, |acc, v| acc + v.2) / count;
    let std = casing_map
        .values()
        .fold(0.0, |acc, v| (acc + (v.2 - avg).powi(2)) / count)
        .sqrt();
    casing_map
        .into_iter()
        .map(|(word, (_, _, all))| (word, all / (avg + std)))
        .collect()
}

/**
 * Formula:
 * WPos = ln(ln(3 + med(pos)))
**/
fn calculate_positional<'a>(words: &'a [&'a str]) -> HashMap<String, f32> {
    words
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut pm, (i, w)| {
            let value = pm.entry(w.to_lowercase()).or_insert(Vec::new());
            value.push(i);
            pm
        })
        .into_iter()
        .map(|(word, positions)| {
            let length = positions.len();
            let median = if length % 2 == 0 {
                let mid = length / 2;
                (positions[mid] + positions[mid - 1]) as f32 / 2.0
            } else {
                positions[length / 2] as f32
            };

            (word, (3.0 + median).ln().ln())
        })
        .collect()
}

fn generate_sentence_sets<'a>(sentences: &'a [Vec<&'a str>]) -> Vec<HashSet<String>> {
    sentences
        .iter()
        .map(|sentence| sentence.iter().map(|w| w.to_lowercase()).collect())
        .collect()
}

/**
 * Formula:
 * WDif = Unique Sentences with word / Total Sentences
**/
fn calculate_different_sentences<'a>(
    sentences: &'a [Vec<&'a str>],
    word: &'a [&'a str],
) -> HashMap<String, f32> {
    let sentences_set = generate_sentence_sets(sentences);
    let words_set = word
        .iter()
        .map(|w| w.to_lowercase())
        .collect::<HashSet<String>>();
    let length = sentences.len();

    words_set
        .into_iter()
        .fold(HashMap::new(), |mut diff_map, word| {
            let value = sentences_set.iter().fold(0.0, |acc, sentence| {
                if sentence.contains(&word) {
                    return acc + 1.0;
                }
                acc
            });
            diff_map.insert(word, value / length as f32);
            diff_map
        })
}

/**
 * Formula:
 * WRel = 0.5 * ((Wdl/Wil) + (Wdr/Wir))
**/
fn calculate_relatedness<'a>(
    diff: &'a HashMap<String, f32>,
    sentences: &'a [Vec<&'a str>],
    ngram: usize,
) -> HashMap<String, f32> {
    todo!()
}
