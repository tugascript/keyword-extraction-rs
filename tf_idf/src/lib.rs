// Copyright (C) 2023 Afonso Barracha
//
// This file is part of keyword-extraction.
//
// keyword-extraction is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// keyword-extraction is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with keyword-extraction.  If not, see <http://www.gnu.org/licenses/>.

use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

use tokenizer::Tokenizer;

pub struct TfIdf(HashMap<String, f64>);

pub enum DocumentSplit {
    Paragraph,
    Sentence,
}

fn generate_word_hashmap(documents: &Vec<String>) -> HashMap<String, f64> {
    let mut word_hashmap: HashMap<String, f64> = HashMap::new();

    for document in documents {
        let words = document.split_whitespace();

        for word in words {
            let count = word_hashmap.entry(word.to_string()).or_insert(0.0);
            *count += 1.0;
        }
    }

    word_hashmap
}

fn generate_unique_word_hashmap(documents: &Vec<String>) -> HashMap<String, f64> {
    let mut unique_word_hashmap: HashMap<String, f64> = HashMap::new();

    for document in documents {
        let unique_words: HashSet<String> = HashSet::from_iter(
            document
                .split_whitespace()
                .into_iter()
                .map(|s| s.to_owned()),
        );

        for word in unique_words {
            let count = unique_word_hashmap.entry(word).or_insert(0.0);
            *count += 1.0;
        }
    }

    unique_word_hashmap
}

fn calculate_tf(mut tf: HashMap<String, f64>) -> HashMap<String, f64> {
    let mut total_words: f64 = 0.0;

    for (_, count) in &tf {
        total_words += *count;
    }

    for (_, count) in &mut tf {
        *count /= total_words;
    }

    tf
}

fn calculate_idf(docs_len: f64, word_hashmap: &HashMap<String, f64>, word: &str) -> f64 {
    let term = word_hashmap.get(word);

    if let Some(term) = term {
        let documents_with_term = docs_len / term;
        documents_with_term.log2()
    } else {
        0.0
    }
}

impl TfIdf {
    pub fn new(text: &str, stopwords: Vec<String>, doc_split: DocumentSplit, punctuation: Option<Vec<String>>) -> TfIdf {
        let documents = match doc_split {
            DocumentSplit::Paragraph => Tokenizer::new(text, stopwords, punctuation).split_into_paragraphs(),
            DocumentSplit::Sentence => Tokenizer::new(text, stopwords, punctuation).split_into_sentences(),
        };
        let tf = calculate_tf(generate_word_hashmap(&documents));
        let docs_len = documents.len() as f64;
        let unique_words_map = generate_unique_word_hashmap(&documents);
        let mut tfidf: HashMap<String, f64> = HashMap::new();

        for (word, value) in &tf {
            let idf = calculate_idf(docs_len, &unique_words_map, word);
            let tfidf_value = value * idf;
            tfidf.insert(word.to_string(), tfidf_value);
        }

        Self(tfidf)
    }

    pub fn get_score(&self, word: &str) -> f64 {
        let score = self.0.get(word);

        if let Some(score) = score {
            *score
        } else {
            0.0
        }
    }

    pub fn get_n_best(&self, n: usize) -> Vec<(String, f64)> {
        let mut sorted_tfidf: Vec<(String, f64)> = self
            .0
            .iter()
            .map(|(word, score)| (word.to_owned(), *score))
            .collect();
        sorted_tfidf.sort_by(|a, b| {
            if let Some(ordering) = b.1.partial_cmp(&a.1) {
                return ordering;
            }

            Ordering::Equal
        });

        if n == 0 || n >= sorted_tfidf.len() {
            return sorted_tfidf;
        }

        sorted_tfidf.truncate(n);
        sorted_tfidf
    }
}
