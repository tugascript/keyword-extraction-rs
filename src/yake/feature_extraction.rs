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
    // Weight (Page 3 of the paper)
    weight: f32,
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

fn calculate_casing(casing_map: CasingMap, weights: TfCasing) -> HashMap<String, f32> {
    let (total_caps, total_upper, total) = casing_map.values().fold((0.0, 0.0, 0.0), |acc, v| {
        (acc.0 + v.0, acc.1 + v.1, acc.2 + v.2)
    });
    casing_map
        .into_iter()
        .map(|(word, (caps, upper, all))| {
            let value = weights.0 * (caps / total_caps)
                + weights.1 * (upper / total_upper)
                + weights.2 * (all / total);
            (word, value / total)
        })
        .collect()
}

fn calculate_tf(casing_map: CasingMap) -> HashMap<String, f32> {
    let total = casing_map.values().fold(0.0, |acc, v| acc + v.2);
    casing_map
        .into_iter()
        .map(|(word, (_, _, all))| (word, all / total))
        .collect()
}

fn calculate_words_positional<'a>(words: &'a [&'a str]) -> HashMap<String, f32> {
    // The beggining of the sentence is the most important (paper page 2)
    let length = words.len();
    words
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut pm, (i, w)| {
            let value = pm.entry(w.to_lowercase()).or_insert((0.0_f32, 0.0_f32));
            value.0 += (length - i) as f32;
            value.1 += 1.0;
            pm
        })
        .into_iter()
        .map(|(word, (pos, tf))| (word, pos / tf))
        .collect()
}

fn calculate_positional<'a>(words: &'a [&'a str]) -> HashMap<String, f32> {
    let words_map = calculate_words_positional(words);
    let max_pos = words_map
        .iter()
        .map(|(_, v)| *v)
        .max_by(|a, b| match a.partial_cmp(b) {
            Some(order) => order,
            None => std::cmp::Ordering::Equal,
        })
        .unwrap_or(1.0);
    words_map
        .into_iter()
        .map(|(word, pos)| (word, pos / max_pos))
        .collect()
}

fn generate_sentence_sets<'a>(sentences: &'a [Vec<&'a str>]) -> Vec<HashSet<String>> {
    sentences
        .iter()
        .map(|sentence| sentence.iter().map(|w| w.to_lowercase()).collect())
        .collect()
}

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

fn calculate_relatedness<'a>(
    diff: &'a HashMap<String, f32>,
    sentences: &'a [Vec<&'a str>],
    ngram: usize,
) -> HashMap<String, f32> {
    let rel = ContextBuilder::new(sentences, ngram)
        .build()
        .into_iter()
        .map(|(word, set)| (word, set.len() as f32))
        .collect::<HashMap<String, f32>>();
    let max = rel
        .iter()
        .map(|(_, v)| *v)
        .max_by(|a, b| match a.partial_cmp(b) {
            Some(order) => order,
            None => std::cmp::Ordering::Equal,
        })
        .unwrap_or(1.0);
    rel.into_iter()
        .map(|(word, value)| {
            let diff_value = diff.get(&word).unwrap_or(&0.0);
            (word, (1.0 - (value / max)) * diff_value)
        })
        .collect()
}
