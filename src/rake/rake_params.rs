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

use crate::common::{PhraseLength, Punctuation, Stopwords, Text};

/// The `RakeParams` enum represents the parameters for the RAKE (Rapid Automatic Keyword Extraction) algorithm.
/// It has two variants: `WithDefaults` and `All`.
pub enum RakeParams<'a> {
    /// The `WithDefaults` variant is used when the user wants to use default values for punctuation and phrase length.
    ///
    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    ///
    /// ### Default values
    /// * `Punctuation` - A list of punctuation symbols, defaults to those in Latin and Germanic languages.
    /// * `PhraseLength` - Optional maximum length of the phrases to be ranked by the RAKE algorithm. Defaults to None.
    WithDefaults(Text<'a>, Stopwords<'a>),

    /// The `WithDefaultsAndPhraseLength` variant is used when the user wants to use default values for punctuation
    /// and specify the phrase length.
    ///
    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `phrase_length` - Optional maximum length of the phrases to be ranked by the RAKE algorithm.
    ///
    /// ### Default values
    /// * `Punctuation` - A list of punctuation symbols, defaults to those in Latin and Germanic languages.
    WithDefaultsAndPhraseLength(Text<'a>, Stopwords<'a>, PhraseLength),

    /// The `All` variant is used when the user wants to specify all parameters.
    ///
    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `punctuation` - Optional list of punctuation symbols.
    /// 4. `phrase_length` - Optional maximum length of the phrases to be ranked by the RAKE algorithm.
    All(Text<'a>, Stopwords<'a>, Punctuation<'a>, PhraseLength),
}

impl<'a> RakeParams<'a> {
    pub fn get_rake_params(self) -> (Text<'a>, Stopwords<'a>, Punctuation<'a>, PhraseLength) {
        match self {
            RakeParams::WithDefaults(text, stop_words) => (text, stop_words, None, None),
            RakeParams::WithDefaultsAndPhraseLength(text, stop_words, phrase_length) => {
                (text, stop_words, None, phrase_length)
            }
            RakeParams::All(text, stop_words, punctuation, phrase_length) => {
                (text, stop_words, punctuation, phrase_length)
            }
        }
    }
}
