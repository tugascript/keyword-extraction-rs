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

use crate::common::{Stopwords, Text};

type WindowSize = usize;
type DampingFactor = f32;
type Tolerance = f32;

/// The parameters to be used in the TextRank algorithm.
pub enum TextRankParams<'a> {
    /// ### Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// ### Default values:
    /// 3. `window_size` - The size of the window to be used in the graph, defaults to 2.
    /// 4. `damping_factor` - The damping factor to be used in the graph, defaults to 0.85.
    /// 5. `tolerance` - The minimum difference between iterations to stop the algorithm, defaults to 0.00005.
    WithDefaults(Text<'a>, Stopwords<'a>),
    /// ### Arguments
    /// 1. `text` - The text to be analyzed.
    /// 2. `stop_words` - A list of stop words.
    /// 3. `window_size` - The size of the window to be used in the graph.
    /// 4. `damping_factor` - The damping factor to be used in the graph.
    /// 5. `tolerance` - The minimum difference between iterations to stop the algorithm.
    All(
        Text<'a>,
        Stopwords<'a>,
        WindowSize,
        DampingFactor,
        Tolerance,
    ),
}

impl<'a> TextRankParams<'a> {
    /// Returns the params to be used in the TextRank algorithm.
    pub fn get_params(&self) -> (Text, Stopwords, WindowSize, DampingFactor, Tolerance) {
        match self {
            TextRankParams::WithDefaults(text, stop_words) => (text, stop_words, 2, 0.85, 0.00005),
            TextRankParams::All(text, stop_words, window_size, damping_factor, min_diff) => {
                (text, stop_words, *window_size, *damping_factor, *min_diff)
            }
        }
    }
}
