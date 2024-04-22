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

use std::collections::HashMap;

use super::sentences_builder::Sentence;

pub type LeftRightContext<'a> = HashMap<&'a str, (Vec<&'a str>, Vec<&'a str>)>;

pub struct ContextBuilder;

impl<'a> ContextBuilder {
    pub fn build_context(
        sentences: &'a [Sentence<'a>],
        window_size: usize,
    ) -> LeftRightContext<'a> {
        sentences
            .iter()
            .fold(LeftRightContext::new(), |mut acc, sentence| {
                sentence.words.iter().enumerate().fold(
                    Vec::<(&str, usize)>::new(),
                    |mut buffer, (i, w1)| {
                        let w1_str = w1.as_ref();

                        buffer.iter().for_each(|(w2, j)| {
                            let entry_1 = acc
                                .entry(sentence.stemmed[i].as_str())
                                .or_insert((Vec::new(), Vec::new()));
                            entry_1.0.push(*w2);
                            let entry_2 = acc
                                .entry(sentence.stemmed[*j].as_str())
                                .or_insert((Vec::new(), Vec::new()));
                            entry_2.1.push(w1_str);
                        });

                        buffer.push((w1_str, i));

                        if buffer.len() > window_size {
                            buffer.remove(0);
                        }

                        buffer
                    },
                );
                acc
            })
    }
}
