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

use crate::{
    common::{Documents, Punctuation, Stopwords, Text},
    tokenizer::Tokenizer,
};

use super::document_processor::DocumentProcessor;

/// The options to split the text into documents.
pub enum TextSplit {
    Sentences,
    Paragraphs,
    Phrases,
}

/// The parameters to be used in the Tf-Idf algorithm.
pub enum TfIdfParams<'a> {
    /// ### Arguments
    /// 1. `documents` - The documents to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `punctuation` - A list of punctuation.
    UnprocessedDocuments(Documents<'a>, Stopwords<'a>, Punctuation<'a>),
    /// ### Arguments
    /// 1. `documents` - The pre-processed documents to be analyzed.
    ProcessedDocuments(Documents<'a>),
    /// ### Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `split` - The option to split the text into documents.
    TextBlock(Text<'a>, Stopwords<'a>, TextSplit),
}

impl<'a> TfIdfParams<'a> {
    /// Returns the documents to be analyzed.
    pub fn get_documents(&self) -> Vec<String> {
        match self {
            TfIdfParams::UnprocessedDocuments(documents, stopwords, punctuatuion) => {
                DocumentProcessor::new(documents, stopwords, punctuatuion).process_documents()
            }
            TfIdfParams::ProcessedDocuments(documents) => documents.to_vec(),
            TfIdfParams::TextBlock(text, stop_words, split) => {
                let tokenizer = Tokenizer::new(text, stop_words, None);
                match split {
                    TextSplit::Sentences => tokenizer.split_into_sentences(),
                    TextSplit::Paragraphs => tokenizer.split_into_paragraphs(),
                    TextSplit::Phrases => tokenizer.split_into_phrases(),
                }
            }
        }
    }
}
