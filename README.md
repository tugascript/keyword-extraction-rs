# Rust Keyword Extraction

## Introduction

This is a simple NLP library with a list of algorithms related to keyword extraction:

- Tokenizer for tokenizing text;
- TF-IDF for calculating the importance of a word in one or more documents;
- Co-occurrence for calculating relationships between words within a specific window size;
- RAKE for extracting key phrases from a document;

And more to come! This library is still in development, and is part of a blog series
on [dev.to](https://dev.to/tugascript).

## Features

The full list of intended features before publishing this library on crates.io is as follows:

- Helper modules:
    - [x] Tokenizer
    - [x] Co-occurrence
- Keyword extraction algorithms:
    - [x] TF-IDF (Needs modifications, a.k.a support for single document and non-processed text)
    - [x] RAKE
    - [ ] TextRank
    - [ ] KEA

I will remove YAKE from the list, as there is already a very good implementation of it in Rust.

## Usage

This library is not yet published on crates.io, so you will have to clone this repository and add it as a dependency in
your `Cargo.toml` file.

## License

This project is licensed under the GNU Lesser General Public License v3.0. See the [Copying](COPYING.md)
and [Copying Lesses](COPYING.LESSER.md) files for details.
