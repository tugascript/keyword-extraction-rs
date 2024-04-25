# Rust Keyword Extraction

## Introduction

This is a simple NLP library with a list of unsupervised keyword extraction algorithms:

- Tokenizer for tokenizing text;
- TF-IDF for calculating the importance of a word in one or more documents;
- Co-occurrence for calculating relationships between words within a specific window size;
- RAKE for extracting key phrases from a document;
- TextRank for extracting keywords and key phrases from a document;
- YAKE for extracting keywords with a n-gram size (defaults to 3) from a document.

## Algorithms

The full list of the algorithms in this library:

- Helper algorithms:
  - [x] Tokenizer
  - [x] Co-occurrence
- Keyword extraction algorithms:
  - [x] TF-IDF
  - [x] RAKE
  - [x] TextRank
  - [x] YAKE

## Usage

Add the library to your `Cargo.toml`:

```toml
[dependencies]
keyword_extraction = "1.4.0"
```

Or use cargo add:

```bash
cargo add keyword_extraction
```

### Features

It is possible to enable or disable features:

- `"tf_idf"`: TF-IDF algorithm;
- `"rake"`: RAKE algorithm;
- `"text_rank"`: TextRank algorithm;
- `"yake"`: YAKE algorithm;
- `"all"`: algorimths and helpers;
- `"parallel"`: parallelization of the algorithms with Rayon;
- `"co_occurrence"`: Co-occurrence algorithm;

Default features: `["tf_idf", "rake", "text_rank"]`. By default all algorithms apart from `"co_occurrence"` and `"yake"` are enabled.

<small>NOTE: `"parallel"` feature is only recommended for large documents, it exchanges memory for computation resourses.</small>

### Examples

For the stop words, you can use the `stop-words` crate:

```toml
[dependencies]
stop-words = "0.8.0"
```

For example for english:

```rust
use stop_words::{get, LANGUAGE};

fn main() {
    let stop_words = get(LANGUAGE::English);
    let punctuation: Vec<String> =[
        ".", ",", ":", ";", "!", "?", "(", ")", "[", "]", "{", "}", "\"", "'",
    ].iter().map(|s| s.to_string()).collect();
    ]
    // ...
}
```

#### TF-IDF

Create a `TfIdfParams` enum which can be one of the following:

1. Unprocessed Documents: `TfIdfParams::UnprocessedDocuments`;
2. Processed Documents: `TfIdfParams::ProcessedDocuments`;
3. Single Unprocessed Document/Text block: `TfIdfParams::TextBlock`;

```rust
use keyword_extraction::tf_idf::{TfIdf, TfIdfParams};

fn main() {
    // ... stop_words
    let documents: Vec<String> = vec![
        "This is a test document.".to_string(),
        "This is another test document.".to_string(),
        "This is a third test document.".to_string(),
    ];

    let params = TfIdfParams::UnprocessedDocuments(&documents, &stop_words, Some(&punctuation));

    let tf_idf = TfIdf::new(params);
    let ranked_keywords: Vec<String> = tf_idf.get_ranked_words(10);
    let ranked_keywords_scores: Vec<(String, f32)> = tf_idf.get_ranked_word_scores(10);

    // ...
}
```

#### RAKE

Create a `RakeParams` enum which can be one of the following:

1. With defaults: `RakeParams::WithDefaults`;
2. With defaults and phrase length (phrase window size limit): `RakeParams::WithDefaultsAndPhraseLength`;
3. All: `RakeParams::All`;

```rust
use keyword_extraction::rake::{Rake, RakeParams};

fn main() {
    // ... stop_words
    let text = r#"
        This is a test document.
        This is another test document.
        This is a third test document.
    "#;

    let rake = Rake::new(RakeParams::WithDefaults(text, &stop_words));
    let ranked_keywords: Vec<String> = rake.get_ranked_words(10);
    let ranked_keywords_scores: Vec<(String, f32)> = rake.get_ranked_word_scores(10);

    // ...
}
```

#### TextRank

Create a `TextRankParams` enum which can be one of the following:

1. With defaults: `TextRankParams::WithDefaults`;
2. With defaults and phrase length (phrase window size limit): `TextRankParams::WithDefaultsAndPhraseLength`;
3. All: `TextRankParams::All`;

```rust
use keyword_extraction::text_rank::{TextRank, TextRankParams};

fn main() {
    // ... stop_words
    let text = r#"
        This is a test document.
        This is another test document.
        This is a third test document.
    "#;

    let text_rank = TextRank::new(TextRankParams::WithDefaults(text, &stop_words));
    let ranked_keywords: Vec<String> = text_rank.get_ranked_words(10);
    let ranked_keywords_scores: Vec<(String, f32)> = text_rank.get_ranked_word_scores(10);
}
```

#### YAKE

**NOTE:** YAKE is a more complex algorithm and doesn't support the `parallel` feature yet.

Create a `YakeParams` enum which can be one of the following:

1. With defaults: `TextRankParams::WithDefaults`;
2. All: `TextRankParams::All`;

```rust
use keyword_extraction::yake::{Yake, YakeParams};

fn main() {
    // ... stop_words
    let text = r#"
        This is a test document.
        This is another test document.
        This is a third test document.
    "#;

    let yake = Yake::new(YakeParams::WithDefaults(text, &stop_words));
    let ranked_keywords: Vec<String> = yake.get_ranked_keywords(10);
    let ranked_keywords_scores: Vec<(String, f32)> = yake.get_ranked_keyword_scoress(10);
    // ...
}
```

## Contributing

I would love your input! I want to make contributing to this project as easy and transparent as possible, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for details.

## License

This project is licensed under the GNU Lesser General Public License v3.0. See the [Copying](COPYING)
and [Copying Lesser](COPYING.LESSER) files for details.
