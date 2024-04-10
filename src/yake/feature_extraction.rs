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

type TfCaps = f32;
type TfUpper = f32;
type TfAll = f32;
type TfCasing = (TfCaps, TfUpper, TfAll);
type CasingMap = HashMap<String, TfCasing>;

pub struct FeatureExtraction {
    features: HashMap<String, f32>,
}

impl<'a> FeatureExtraction {
    /**
     * Formula:
     *  H = (WPos * WRel) / (WCas + (WFreq/WRel) + (WDif/WRel))
     **/
    pub fn new(
        sentences: &'a [Vec<&'a str>],
        words: &'a [&'a str],
        right_left_context: HashMap<String, Vec<(Vec<&'a str>, Vec<&'a str>)>>,
    ) -> Self {
        let casing_map = generate_casing_map(words);
        let cas = calculate_casing(&casing_map);
        let tf = calculate_tf(&casing_map);
        let pos = calculate_positional(words);
        let rel = calculate_relatedness(sentences, &right_left_context);
        let dif = calculate_different_sentences(sentences, &right_left_context);

        Self {
            features: right_left_context
                .iter()
                .map(|(word, _)| {
                    let word = word.as_str();
                    let wcas = *cas.get(word).unwrap_or(&f32::EPSILON);
                    let wfreq = *tf.get(word).unwrap_or(&0.0);
                    let wpos = *pos.get(word).unwrap_or(&0.0);
                    let wrel = *rel.get(word).unwrap_or(&f32::EPSILON);
                    let wdif = *dif.get(word).unwrap_or(&0.0);

                    (
                        word.to_string(),
                        (wpos * wrel) / (wcas + (wfreq / wrel) + (wdif / wrel)),
                    )
                })
                .collect(),
        }
    }

    pub fn get_feature_score(&self, word: &str) -> Option<f32> {
        self.features.get(word).copied()
    }
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
fn calculate_casing<'a>(casing_map: &'a CasingMap) -> HashMap<&'a str, f32> {
    casing_map
        .iter()
        .map(|(word, (caps, upper, all))| {
            let max = if caps > upper { caps } else { upper };
            (word.as_str(), max / (1.0 + all.ln()))
        })
        .collect()
}

/**
 * Formula:
 * WFreq = TfAll / (avgTf + stdTf)
**/
fn calculate_tf<'a>(casing_map: &'a CasingMap) -> HashMap<&'a str, f32> {
    let count = casing_map.len() as f32 + f32::EPSILON;
    let avg = casing_map.values().fold(0.0, |acc, v| acc + v.2) / count;
    let std = casing_map
        .values()
        .fold(0.0, |acc, v| (acc + (v.2 - avg).powi(2)) / count)
        .sqrt();
    casing_map
        .iter()
        .map(|(word, (_, _, all))| (word.as_str(), all / (avg + std + f32::EPSILON)))
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

/**
 * Formula:
 * WDif = Unique Sentences with word / Total Sentences
**/
fn calculate_different_sentences<'a>(
    sentences: &'a [Vec<&'a str>],
    right_left_context: &'a HashMap<String, Vec<(Vec<&'a str>, Vec<&'a str>)>>,
) -> HashMap<&'a str, f32> {
    let length = sentences.len() as f32 + f32::EPSILON;
    right_left_context
        .iter()
        .map(|(key, val)| (key.as_str(), val.len() as f32 / length))
        .collect()
}

/**
 * Formula:
 * WRel = ((Wdl/Wil) + (Wdr/Wir)) / 2
**/
fn calculate_relatedness<'a>(
    sentences: &'a [Vec<&'a str>],
    right_left_context: &'a HashMap<String, Vec<(Vec<&'a str>, Vec<&'a str>)>>,
) -> HashMap<&'a str, f32> {
    let length = sentences.len() as f32;
    right_left_context
        .iter()
        .map(|(word, contexts)| {
            let context_length = contexts.len() as f32;
            let (left_unique, left_total, right_unique, right_total) = contexts.iter().fold(
                (HashSet::new(), 0.0, HashSet::new(), 0.0),
                |(mut left_unique, mut left_total, mut right_unique, mut right_total),
                 (left, right)| {
                    left.iter().for_each(|w| {
                        left_unique.insert(w.to_lowercase());
                        left_total += 1.0;
                    });
                    right.iter().for_each(|w| {
                        right_unique.insert(w.to_lowercase());
                        right_total += 1.0;
                    });
                    (left_unique, left_total, right_unique, right_total)
                },
            );

            let dif = (context_length / length) + f32::EPSILON;
            let wdl = left_unique.len() as f32 / dif;
            let wdr = right_unique.len() as f32 / dif;
            let wil = left_total + f32::EPSILON;
            let wir = right_total + f32::EPSILON;

            (word.as_str(), ((wdl / wil) + (wdr / wir)) / 2.0)
        })
        .collect()
}
