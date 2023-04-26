// Copyright (C) 2023 Afonso Barracha
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

use std::{collections::HashMap, ops::Range};

pub struct CoOccurrence {
    matrix: Vec<Vec<f32>>,
    words_indexes: HashMap<String, usize>,
    indexes_words: HashMap<usize, String>,
}

fn get_window_range(window_size: usize, index: usize, words_length: usize) -> Range<usize> {
    let window_start = if index < window_size {
        0
    } else {
        index - window_size
    };
    let window_end = if index + window_size + 1 > words_length {
        words_length
    } else {
        index + window_size + 1
    };
    window_start..window_end
}

fn create_words_indexes(words: &[String]) -> HashMap<String, usize> {
    words
        .iter()
        .enumerate()
        .map(|(i, w)| (w.to_string(), i))
        .collect::<HashMap<String, usize>>()
}

fn create_indexes_words(labels: &HashMap<String, usize>) -> HashMap<usize, String> {
    labels
        .iter()
        .map(|(w, i)| (i.to_owned(), w.to_string()))
        .collect::<HashMap<usize, String>>()
}

fn get_matrix(
    documents: &[String],
    words_indexes: &HashMap<String, usize>,
    length: usize,
    window_size: usize,
) -> Vec<Vec<f32>> {
    let mut matrix = vec![vec![0.0_f32; length]; length];
    let mut max = 0.0_f32;

    documents.iter().for_each(|doc| {
        let doc_words = doc.split_whitespace().collect::<Vec<&str>>();
        doc_words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| words_indexes.get(*w).map(|first_index| (i, *first_index)))
            .for_each(|(i, first_index)| {
                get_window_range(window_size, i, doc_words.len())
                    .filter_map(|j| {
                        if i == j {
                            return None;
                        }

                        doc_words
                            .get(j)
                            .and_then(|other_word| words_indexes.get(*other_word))
                            .map(|other_index| (j, *other_index))
                    })
                    .for_each(|(_, other_index)| {
                        matrix[first_index][other_index] += 1.0;
                        let current = matrix[first_index][other_index];

                        if current > max {
                            max = current;
                        }
                    });
            });
    });

    matrix
        .iter_mut()
        .flat_map(|row| row.iter_mut())
        .for_each(|value| *value /= max);
    matrix
}

impl CoOccurrence {
    pub fn new(documents: &[String], words: &[String], window_size: usize) -> Self {
        let words_indexes = create_words_indexes(words);
        let length = words.len();

        Self {
            matrix: get_matrix(documents, &words_indexes, length, window_size),
            indexes_words: create_indexes_words(&words_indexes),
            words_indexes,
        }
    }

    pub fn get_label(&self, word: &str) -> Option<usize> {
        self.words_indexes.get(word).map(|w| w.to_owned())
    }

    pub fn get_word(&self, label: usize) -> Option<String> {
        self.indexes_words.get(&label).map(|w| w.to_owned())
    }

    pub fn get_matrix(&self) -> &Vec<Vec<f32>> {
        &self.matrix
    }

    pub fn get_labels(&self) -> &HashMap<String, usize> {
        &self.words_indexes
    }

    pub fn get_relations(&self, word: &str) -> Option<Vec<(String, f32)>> {
        let label = match self.get_label(word) {
            Some(l) => l,
            None => return None,
        };
        Some(
            self.matrix[label]
                .iter()
                .enumerate()
                .filter_map(|(i, &v)| {
                    if v > 0.0 {
                        if let Some(w) = self.get_word(i) {
                            return Some((w, v));
                        }
                    }

                    None
                })
                .collect::<Vec<(String, f32)>>(),
        )
    }

    pub fn get_matrix_row(&self, word: &str) -> Option<Vec<f32>> {
        let label = match self.get_label(word) {
            Some(l) => l,
            None => return None,
        };
        Some(self.matrix[label].to_owned())
    }

    pub fn get_relation(&self, word1: &str, word2: &str) -> Option<f32> {
        let label1 = match self.get_label(word1) {
            Some(l) => l,
            None => return None,
        };
        let label2 = match self.get_label(word2) {
            Some(l) => l,
            None => return None,
        };
        Some(self.matrix[label1][label2])
    }
}
