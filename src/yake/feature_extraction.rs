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

use super::context_builder::WordContext;

pub struct FeaturedWord<'a> {
    // The word itself
    word: &'a str,
    // Casing
    cas: f32,
    // Frequency
    tf: f32,
    // Positional
    pos: f32,
    // Relatedness
    rel: f32,
    // Difference sentence
    dif: f32,
}

pub struct FeatureExtraction<'a> {
    words: Vec<&'a str>,
    context: WordContext<'a>,
}
