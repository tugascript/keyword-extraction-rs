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

use std::collections::HashSet;

use regex::Regex;

use crate::common::{get_special_char_regex, process_word, PUNCTUATION};

pub struct DocumentProcessor<'a> {
    documents: &'a [String],
    stopwords: HashSet<String>,
    punctuation: HashSet<String>,
}

impl<'a> DocumentProcessor<'a> {
    pub fn new(
        documents: &'a [String],
        stopwords: &'a [String],
        punctuation: &'a Option<&'a [String]>,
    ) -> Self {
        Self {
            documents,
            stopwords: stopwords
                .iter()
                .map(|s| s.to_owned())
                .collect::<HashSet<String>>(),
            punctuation: punctuation
                .unwrap_or(
                    &PUNCTUATION
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                )
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>(),
        }
    }

    fn process_document(&self, document: &str, special_char_regex: &Regex) -> String {
        document
            .split_whitespace()
            .filter_map(|w| process_word(w, special_char_regex, &self.stopwords, &self.punctuation))
            .collect::<Vec<String>>()
            .join(" ")
    }

    pub fn process_documents(&self) -> Vec<String> {
        let special_char_regex = get_special_char_regex();
        self.documents
            .iter()
            .map(|doc| self.process_document(doc, &special_char_regex))
            .collect::<Vec<String>>()
    }
}
