# Rust Keyword Extraction

## Introduction

This is a simple NLP library with a list of algorithms related to keyword extraction:

- Tokenizer for tokenizing text;
- TF-IDF for calculating the importance of a word in one or more documents;
- Co-occurrence for calculating relationships between words within a specific window size;
- RAKE for extracting key phrases from a document;
- TextRank for extracting keywords and key phrases from a document;

## Algorithms

The full list of the algorithms in this library:

- Helper algorithms:
    - [x] Tokenizer
    - [x] Co-occurrence
- Keyword extraction algorithms:
    - [x] TF-IDF
    - [x] RAKE
    - [x] TextRank

Upcoming algorithms:
- YAKE

## Usage

Add the library to your `Cargo.toml`:

```toml
[dependencies]
keyword-extraction = "1.2.0"
```

Or use cargo add:

```bash
cargo add keyword-extraction
```

### Features

It is possible to enable or disable features:

- `"tf_idf"`: TF-IDF algorithm;
- `"rake"`: RAKE algorithm;
- `"text_rank"`: TextRank algorithm;
- `"all"`: all algorithms;
- `"parallel"`: parallelization of the algorithms with Rayon;

Default features: `"tf_idf"`.

NOTE: `"parallel"` feature is only recommended for large documents, it exchanges memory for computation resourses.

## License

This project is licensed under the GNU Lesser General Public License v3.0. See the [Copying](COPYING)
and [Copying Lesser](COPYING.LESSER) files for details.
