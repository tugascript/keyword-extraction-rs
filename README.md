# Rust Keyword Extraction

## Introduction

This is a simple NLP library with a list of algorithms related to keyword extraction:

- Tokenizer for tokenizing text;
- TF-IDF for calculating the importance of a word in one or more documents;
- Co-occurrence for calculating relationships between words within a specific window size;
- RAKE for extracting key phrases from a document;
- TextRank for extracting keywords and key phrases from a document;

## Features

The full list of intended features before publishing this library on crates.io is as follows:

- Helper modules:
    - [x] Tokenizer
    - [x] Co-occurrence
- Keyword extraction algorithms:
    - [x] TF-IDF
    - [x] RAKE
    - [x] TextRank
    - [ ] YAKE (simplified, without case sensitivity and stemming)

## Usage

This library is not yet published on crates.io, so you will have to clone this repository and add it as a dependency in
your `Cargo.toml` file.

## License

This project is licensed under the GNU Lesser General Public License v3.0. See the [Copying](COPYING.md)
and [Copying Lesses](COPYING.LESSER.md) files for details.
