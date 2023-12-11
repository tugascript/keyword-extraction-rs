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

/// The `TfIdfParams` enum represents the parameters for the TF-IDF (Term Frequency - Inverse Document Frequency) algorithm.
/// The parameters to be used in the Tf-Idf algorithm.
pub enum TfIdfParams<'a> {
    /// Represents unprocessed documents to be analyzed.
    ///
    /// ## Arguments
    /// * `documents`: The documents to be analyzed.
    /// * `stop_words`: A list of stop words.
    /// * `punctuation`: Optional list of punctuation symbols.
    UnprocessedDocuments(Documents<'a>, Stopwords<'a>, Punctuation<'a>),

    /// Represents pre-processed documents to be analyzed.
    ///
    /// ## Arguments
    /// * `documents`: The pre-processed documents to be analyzed.
    ProcessedDocuments(Documents<'a>),

    /// Represents a text block to be analyzed.
    ///
    /// ## Arguments
    /// * `text`: The text to be analyzed.
    /// * `stop_words`: A list of stop words.
    /// * `punctuation`: Optional list of punctuation symbols.
    /// * `split`: The option to split the text into documents.
    TextBlock(Text<'a>, Stopwords<'a>, Punctuation<'a>, TextSplit),
}

impl<'a> TfIdfParams<'a> {
    /// Returns the documents to be analyzed.
    pub fn get_documents(&self) -> Vec<String> {
        match self {
            TfIdfParams::UnprocessedDocuments(documents, stopwords, punctuatuion) => {
                DocumentProcessor::new(documents, stopwords, punctuatuion).process_documents()
            }
            TfIdfParams::ProcessedDocuments(documents) => documents.to_vec(),
            TfIdfParams::TextBlock(text, stop_words, punctuation, split) => {
                let tokenizer = Tokenizer::new(text, stop_words, *punctuation);
                match split {
                    TextSplit::Sentences => tokenizer.split_into_sentences(),
                    TextSplit::Paragraphs => tokenizer.split_into_paragraphs(),
                    TextSplit::Phrases => tokenizer.split_into_phrases(None),
                }
            }
        }
    }
}
