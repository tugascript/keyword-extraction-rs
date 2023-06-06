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

use std::collections::HashMap;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub struct TextRankLogic;

fn score_frase(phrase: &str, word_rank: &HashMap<String, f32>) -> (String, f32) {
    let words = phrase.split_whitespace().collect::<Vec<&str>>();
    let score = words
        .iter()
        .filter_map(|word| word_rank.get(*word))
        .sum::<f32>();

    (phrase.to_string(), score / words.len() as f32)
}

fn score_word(
    edges: &HashMap<String, f32>,
    node_indexes: &HashMap<String, usize>,
    outgoing_weight_sums: &HashMap<String, f32>,
    prev_scores: &[f32],
    damping: f32,
) -> f32 {
    let new_score = edges
        .iter()
        .map(|(neighbor, weight)| {
            let neighbor_index = node_indexes[neighbor];
            let neighbor_outgoing_sum = outgoing_weight_sums[neighbor];
            weight / neighbor_outgoing_sum * prev_scores[neighbor_index]
        })
        .sum::<f32>();

    (1.0 - damping) + damping * new_score
}

fn get_node_indexes(nodes: &[&String]) -> HashMap<String, usize> {
    #[cfg(feature = "parallel")]
    return nodes
        .par_iter()
        .enumerate()
        .map(|(i, w)| (w.to_string(), i))
        .collect::<HashMap<String, usize>>();

    #[cfg(not(feature = "parallel"))]
    nodes
        .iter()
        .enumerate()
        .map(|(i, w)| (w.to_string(), i))
        .collect::<HashMap<String, usize>>()
}

fn get_scores(
    graph: &HashMap<String, HashMap<String, f32>>,
    node_indexes: &HashMap<String, usize>,
    outgoing_weight_sums: &HashMap<String, f32>,
    prev_scores: &[f32],
    damping: f32,
) -> Vec<f32> {
    #[cfg(feature = "parallel")]
    return graph
        .par_iter()
        .map(|(_, edges)| {
            score_word(
                edges,
                &node_indexes,
                &outgoing_weight_sums,
                &prev_scores,
                damping,
            )
        })
        .collect();

    #[cfg(not(feature = "parallel"))]
    graph
        .values()
        .map(|edges| {
            score_word(
                edges,
                &node_indexes,
                &outgoing_weight_sums,
                &prev_scores,
                damping,
            )
        })
        .collect();
}

fn check_tolorance(scores: &[f32], prev_scores: &[f32], tol: f32) -> bool {
    #[cfg(feature = "parallel")]
    return scores.par_iter().enumerate().all(|(i, score)| {
        let prev_score = prev_scores[i];
        (score - prev_score).abs() < tol
    });

    #[cfg(not(feature = "parallel"))]
    scores
        .iter()
        .zip(prev_scores.iter())
        .all(|(score, prev_score)| (score - prev_score).abs() < tol)
}

impl TextRankLogic {
    pub fn build_text_rank(
        words: Vec<String>,
        phrases: Vec<String>,
        window_size: usize,
        damping: f32,
        tol: f32,
    ) -> (HashMap<String, f32>, HashMap<String, f32>) {
        let word_rank =
            Self::create_word_rank(Self::create_graph(words, window_size), damping, tol);
        let phrase_rank = Self::rank_phrases(phrases, &word_rank);
        (word_rank, phrase_rank)
    }

    fn add_edge(graph: &mut HashMap<String, HashMap<String, f32>>, word1: &str, word2: &str) {
        graph
            .entry(word1.to_string())
            .or_insert_with(HashMap::new)
            .entry(word2.to_string())
            .and_modify(|e| *e += 1.0)
            .or_insert(1.0);
    }

    fn create_graph(
        words: Vec<String>,
        window_size: usize,
    ) -> HashMap<String, HashMap<String, f32>> {
        let mut graph = HashMap::new();

        words
            .iter()
            .enumerate()
            .flat_map(|(i, word1)| {
                words[i + 1..]
                    .iter()
                    .take(window_size)
                    .filter(|word2| word1.as_str() != word2.as_str())
                    .map(move |word2| (word1, word2))
            })
            .for_each(|(word1, word2)| {
                Self::add_edge(&mut graph, word1, word2);
                Self::add_edge(&mut graph, word2, word1);
            });

        graph
    }

    fn get_outgoing_weight_sum(
        graph: &HashMap<String, HashMap<String, f32>>,
    ) -> HashMap<String, f32> {
        #[cfg(feature = "parallel")]
        return graph
            .par_iter()
            .map(|(node, edges)| {
                let outgoing_weight_sum = edges.values().sum();
                (node.to_string(), outgoing_weight_sum)
            })
            .collect();

        #[cfg(not(feature = "parallel"))]
        graph
            .iter()
            .map(|(node, edges)| {
                let outgoing_weight_sum = edges.values().sum();
                (node.to_string(), outgoing_weight_sum)
            })
            .collect()
    }

    fn create_word_rank(
        graph: HashMap<String, HashMap<String, f32>>,
        damping: f32,
        tol: f32,
    ) -> HashMap<String, f32> {
        let nodes = graph.keys().collect::<Vec<&String>>();
        let n = nodes.len();
        let node_indexes = get_node_indexes(&nodes);
        let mut scores = vec![1.0_f32; n];
        let outgoing_weight_sums = Self::get_outgoing_weight_sum(&graph);

        loop {
            let prev_scores = scores.to_owned();
            scores = get_scores(
                &graph,
                &node_indexes,
                &outgoing_weight_sums,
                &prev_scores,
                damping,
            );

            if check_tolorance(&scores, &prev_scores, tol) {
                break;
            }
        }

        #[cfg(feature = "parallel")]
        return nodes
            .par_iter()
            .map(|&node| (node.to_string(), scores[node_indexes[node]]))
            .collect::<HashMap<String, f32>>();

        #[cfg(not(feature = "parallel"))]
        nodes
            .iter()
            .map(|&node| (node.to_string(), scores[node_indexes[node]]))
            .collect::<HashMap<String, f32>>()
    }

    fn rank_phrases(
        phrases: Vec<String>,
        word_rank: &HashMap<String, f32>,
    ) -> HashMap<String, f32> {
        #[cfg(feature = "parallel")]
        return phrases
            .par_iter()
            .map(|phrase| score_frase(phrase, word_rank))
            .collect::<HashMap<String, f32>>();

        #[cfg(not(feature = "parallel"))]
        phrases
            .iter()
            .map(|phrase| score_frase(phrase, word_rank))
            .collect::<HashMap<String, f32>>()
    }
}
