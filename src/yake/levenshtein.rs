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

use std::cmp::{max, min};

use unicode_segmentation::UnicodeSegmentation;

fn calculate_distance(str1: &str, str2: &str) -> usize {
    if (str1.is_empty() && str2.is_empty()) || str1 == str2 {
        return 0;
    }

    let graphemes1 = str1.graphemes(true);
    let graphemes2 = str2.graphemes(true);
    let len = graphemes2.clone().count() + 1;
    let mut prev_row = (0..len).collect::<Vec<usize>>();

    let last_row = graphemes1
        .enumerate()
        .fold(prev_row.clone(), |row, (i, char1)| {
            let mut new_row = vec![i + 1; len];
            graphemes2.clone().enumerate().for_each(|(j, char2)| {
                let cost = if char1 == char2 { 0 } else { 1 };
                new_row[j + 1] = min(row[j + 1] + 1, min(new_row[j] + 1, row[j] + cost));
            });
            prev_row = row;
            new_row
        });

    last_row[len - 1]
}

pub struct Levenshtein<'a>(&'a str, &'a str, usize);

impl<'a> Levenshtein<'a> {
    pub fn new(str1: &'a str, str2: &'a str) -> Self {
        Self(str1, str2, calculate_distance(str1, str2))
    }

    pub fn ratio(&self) -> f32 {
        let max_len = max(
            self.0.graphemes(true).count(),
            self.1.graphemes(true).count(),
        );
        1.0 - (self.2 as f32 / max_len as f32)
    }
}
