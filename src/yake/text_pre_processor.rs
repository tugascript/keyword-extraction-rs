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

use std::borrow::Cow;

use crate::common::get_space_regex;

pub struct TextPreProcessor;

impl<'a> TextPreProcessor {
    pub fn process_text(text: &'a str) -> Cow<'a, str> {
        let space_regex = get_space_regex();
        let trimmed_text = text.trim();

        if let Some(regex) = space_regex {
            regex.replace_all(trimmed_text, " ")
        } else {
            trimmed_text.into()
        }
    }
}
