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

use crate::common::{Punctuation, Stopwords, Text, WindowSize};

type Threshold = f32;
type Ngram = usize;

pub enum YakeParams<'a> {
    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// ### Defaults values:
    /// * `punctuation` - A list of punctuation symbols, defaults to those in Latin and Germanic languages.
    /// * `threshold` - 0.85
    /// * `ngram` - 3
    /// * `window_size` - 2
    WithDefaults(Text<'a>, Stopwords<'a>),

    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `threshold` - The threshold to be used for candidate filtering.
    /// 4. `ngram` - The size of the n-grams to be used for keyword.
    /// 5. `window_size` - The size of the window to be used for keyword extraction.
    All(
        Text<'a>,
        Stopwords<'a>,
        Punctuation<'a>,
        Threshold,
        Ngram,
        WindowSize,
    ),
}

impl<'a> YakeParams<'a> {
    pub fn get_params(
        &self,
    ) -> (
        Text<'a>,
        Stopwords<'a>,
        Punctuation<'a>,
        Threshold,
        Ngram,
        WindowSize,
    ) {
        match self {
            YakeParams::WithDefaults(text, stop_words) => (*text, *stop_words, None, 0.85, 3, 2),
            YakeParams::All(text, stop_words, punctuation, threshold, ngram, window_size) => (
                *text,
                *stop_words,
                *punctuation,
                *threshold,
                *ngram,
                *window_size,
            ),
        }
    }
}
