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

use std::collections::HashMap;

use unicode_segmentation::UnicodeSegmentation;

use crate::common::{get_capitalized_regex, get_upper_case_regex};

use super::context_builder::WordContext;

pub struct FeaturedWord {
    // Casing
    cas: f32,
    // Frequency
    tf: f32,
    // Positional
    pos: f32,
    // Relatedness
    rel: f32,
    // Difference sentence
    dif: f32,
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

fn calculate_tf(casing_map: CasingMap) -> HashMap<String, f32> {
    let total = casing_map.values().fold(0.0, |acc, v| acc + v.2);
    casing_map
        .into_iter()
        .map(|(word, (_, _, all))| (word, all / total))
        .collect()
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

fn calculate_words_positional<'a>(words: &'a [&'a str]) -> HashMap<String, f32> {
    words
        .iter()
        .enumerate()
        .fold(HashMap::new(), |mut pm, (i, w)| {
            let value = pm.entry(w.to_lowercase()).or_insert((0.0_f32, 0.0_f32));
            value.0 += i as f32;
            value.1 += 1.0;
            pm
        })
        .into_iter()
        .map(|(word, (pos, tf))| (word, pos / tf))
        .collect()
}

fn calculate_sentence_positional<'a>(sentences: &'a [Vec<&'a str>]) -> HashMap<String, f32> {
    sentences
        .iter()
        .map(|sentence| calculate_words_positional(sentence))
        .fold(HashMap::new(), |mut spm, pm| {
            for (word, pos) in pm {
                let value = spm.entry(word).or_insert((0.0_f32, 0.0_f32));
                value.0 += pos;
                value.1 += 1.0;
            }
            spm
        })
        .into_iter()
        .map(|(word, (pos, sf))| (word, pos / sf))
        .collect()
}

fn calculate_positional<'a>(
    words: &'a [&'a str],
    sentences: &'a [Vec<&'a str>],
) -> HashMap<String, f32> {
    let words_map = calculate_words_positional(words);
    let sentences_map = calculate_sentence_positional(sentences);
    let pos_map = words_map
        .into_iter()
        .map(|(word, pos)| {
            let sentence_pos = sentences_map.get(&word).unwrap_or(&0.0);
            (word, pos - sentence_pos)
        })
        .collect::<HashMap<String, f32>>();
    let max_pos = *pos_map
        .values()
        .max_by(|a, b| match a.partial_cmp(b) {
            Some(order) => order,
            None => std::cmp::Ordering::Equal,
        })
        .unwrap_or(&1.0);
    pos_map
        .into_iter()
        .map(|(word, pos)| (word, pos / max_pos))
        .collect()
}

// TODO: Implement difference sentence (sentence levenstein distance)
// TODO: Implement relatedness (co-occurrence)
