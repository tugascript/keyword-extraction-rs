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

use std::collections::{HashMap, HashSet};

pub type WordContext<'a> = HashMap<String, (HashSet<String>, HashSet<String>)>;

pub struct ContextBuilder<'a> {
    sentences: &'a [Vec<&'a str>],
    window_size: usize,
}

impl<'a> ContextBuilder<'a> {
    pub fn new(sentences: &'a [Vec<&'a str>], window_size: usize) -> Self {
        Self {
            sentences,
            window_size,
        }
    }

    pub fn build(&self) -> WordContext<'a> {
        self.sentences
            .iter()
            .fold(HashMap::new(), |mut ctx, sentence| {
                sentence.iter().enumerate().for_each(|(i, word)| {
                    let ctx = ctx
                        .entry(word.to_lowercase())
                        .or_insert((HashSet::new(), HashSet::new()));
                    sentence
                        .iter()
                        .take(i)
                        .rev()
                        .take(self.window_size)
                        .rev()
                        .for_each(|word| {
                            ctx.0.insert(word.to_lowercase());
                        });
                    sentence
                        .iter()
                        .skip(i + 1)
                        .take(self.window_size)
                        .for_each(|word| {
                            ctx.1.insert(word.to_lowercase());
                        });
                });
                ctx
            })
    }
}
