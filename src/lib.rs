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

mod common;

#[cfg(feature = "co_occurrence")]
pub mod co_occurrence;

#[cfg(feature = "rake")]
pub mod rake;

#[cfg(feature = "text_rank")]
pub mod text_rank;

#[cfg(feature = "tf_idf")]
pub mod tf_idf;

#[cfg(feature = "yake")]
pub mod yake;

pub mod tokenizer;

#[cfg(test)]
mod tests;
