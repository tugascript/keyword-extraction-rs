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

use crate::common::{PhraseLength, Punctuation, Stopwords, Text, WindowSize};

type DampingFactor = f32;
type Tolerance = f32;

/// The parameters to be used in the TextRank algorithm.
pub enum TextRankParams<'a> {
    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// ### Default values:
    /// * `punctuation` - A list of punctuation symbols, defaults to those in Latin and Germanic languages.
    /// * `window_size` - The size of the window to be used in the graph, defaults to 2.
    /// * `damping_factor` - The damping factor to be used in the graph, defaults to 0.85.
    /// * `tolerance` - The minimum difference between iterations to stop the algorithm, defaults to 0.00005.
    WithDefaults(Text<'a>, Stopwords<'a>),

    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `phrase_length` - Optional maximum length of the phrases to be ranked by the RAKE algorithm.
    /// ### Default values:
    /// * `punctuation` - A list of punctuation symbols, defaults to those in Latin and Germanic languages.
    /// * `window_size` - The size of the window to be used in the graph, defaults to 2.
    /// * `damping_factor` - The damping factor to be used in the graph, defaults to 0.85.
    /// * `tolerance` - The minimum difference between iterations to stop the algorithm, defaults to 0.00005.
    WithDefaultsAndPhraseLength(Text<'a>, Stopwords<'a>, PhraseLength),

    /// ## Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `punctuation` - Optional list of punctuation symbols.
    /// 4. `window_size` - The size of the window to be used in the graph.
    /// 5. `damping_factor` - The damping factor to be used in the graph.
    /// 6. `tolerance` - The minimum difference between iterations to stop the algorithm.
    /// 7. `phrase_length` - Optional maximum length of the phrases to be ranked by the RAKE algorithm.
    All(
        Text<'a>,
        Stopwords<'a>,
        Punctuation<'a>,
        WindowSize,
        DampingFactor,
        Tolerance,
        PhraseLength,
    ),
}

impl<'a> TextRankParams<'a> {
    /// Returns the params to be used in the TextRank algorithm.
    pub fn get_params(
        &self,
    ) -> (
        Text,
        Stopwords,
        Punctuation,
        WindowSize,
        DampingFactor,
        Tolerance,
        PhraseLength,
    ) {
        match self {
            TextRankParams::WithDefaults(text, stop_words) => {
                (text, stop_words, None, 2, 0.85, 0.00005, None)
            }
            TextRankParams::WithDefaultsAndPhraseLength(text, stop_words, phrase_length) => {
                (text, stop_words, None, 2, 0.85, 0.00005, *phrase_length)
            }
            TextRankParams::All(
                text,
                stop_words,
                punctuation,
                window_size,
                damping_factor,
                min_diff,
                phrase_length,
            ) => (
                text,
                stop_words,
                *punctuation,
                *window_size,
                *damping_factor,
                *min_diff,
                *phrase_length,
            ),
        }
    }
}
