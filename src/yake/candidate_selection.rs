use std::{
    cmp::min,
    collections::{HashMap, HashSet},
};

use super::yake_tokenizer::Sentence;

type PreCandidate<'a> = HashMap<String, (Vec<&'a str>, Vec<Vec<&'a str>>)>;

fn build_pre_candidates<'a>(sentences: &'a [Sentence], ngram: usize) -> PreCandidate<'a> {
    sentences
        .iter()
        .fold(PreCandidate::new(), |mut acc, sentence| {
            let sentence_len = sentence.get_length();

            (0..sentence_len).for_each(|j| {
                (j + 1..=min(j + ngram, sentence_len)).for_each(|k| {
                    let stems = sentence.get_stemmed()[j..k]
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&'a str>>();
                    let words = sentence.get_words()[j..k]
                        .iter()
                        .copied()
                        .collect::<Vec<&'a str>>();
                    let key = stems.join(" ");
                    let entry = acc.entry(key).or_insert((Vec::new(), Vec::new()));
                    if entry.0.is_empty() {
                        entry.0 = stems;
                    }
                    entry.1.push(words);
                });
            });
            acc
        })
}

fn filter_pre_candidates<'a>(
    mut pre_candidates: PreCandidate<'a>,
    stop_words: &'a HashSet<&'a str>,
) -> PreCandidate<'a> {
    pre_candidates.retain(|_, (s, _)| s.iter().all(|w| !stop_words.contains(w)));
    pre_candidates
}

pub struct Candidates<'a>(PreCandidate<'a>);

impl<'a> Candidates<'a> {
    pub fn new(sentences: &'a [Sentence], ngram: usize, stop_words: &'a HashSet<&'a str>) -> Self {
        Self(filter_pre_candidates(
            build_pre_candidates(sentences, ngram),
            stop_words,
        ))
    }

    pub fn candidates(
        &self,
    ) -> impl Iterator<Item = (&String, &(Vec<&'a str>, Vec<Vec<&'a str>>))> {
        self.0.iter()
    }
}
