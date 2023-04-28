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

pub struct TextRankLogic;

impl TextRankLogic {
    pub fn build_text_rank(
        words: Vec<String>,
        phrases: Vec<String>,
        window_size: usize,
        damping: f32,
        tol: f32,
    ) -> (HashMap<String, f32>, HashMap<String, f32>) {
        let word_rank = TextRankLogic::create_word_rank(
            TextRankLogic::create_graph(words, window_size),
            damping,
            tol,
        );
        let phrase_rank = TextRankLogic::rank_phrases(phrases, &word_rank);
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
                TextRankLogic::add_edge(&mut graph, word1, word2);
                TextRankLogic::add_edge(&mut graph, word2, word1);
            });

        graph
    }

    fn get_outgoing_weight_sum(
        graph: &HashMap<String, HashMap<String, f32>>,
    ) -> HashMap<String, f32> {
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
        let node_indexes = nodes
            .iter()
            .enumerate()
            .map(|(i, w)| (w.to_string(), i))
            .collect::<HashMap<String, usize>>();

        let mut scores = vec![1.0_f32; n];
        let outgoing_weight_sums = TextRankLogic::get_outgoing_weight_sum(&graph);

        loop {
            let prev_scores = scores.to_owned();
            scores = graph
                .values()
                .map(|edges| {
                    let new_score = edges
                        .iter()
                        .map(|(neighbor, weight)| {
                            let neighbor_index = node_indexes[neighbor];
                            let neighbor_outgoing_sum = outgoing_weight_sums[neighbor];
                            weight / neighbor_outgoing_sum * prev_scores[neighbor_index]
                        })
                        .sum::<f32>();

                    (1.0 - damping) + damping * new_score
                })
                .collect();

            if scores
                .iter()
                .zip(prev_scores.iter())
                .all(|(s1, s2)| (s1 - s2).abs() < tol)
            {
                break;
            }
        }

        nodes
            .iter()
            .map(|&node| (node.to_string(), scores[node_indexes[node]]))
            .collect::<HashMap<String, f32>>()
    }

    fn rank_phrases(
        phrases: Vec<String>,
        word_rank: &HashMap<String, f32>,
    ) -> HashMap<String, f32> {
        phrases
            .iter()
            .map(|phrase| {
                let words = phrase.split_whitespace().collect::<Vec<&str>>();
                let score = words
                    .iter()
                    .filter_map(|word| word_rank.get(*word))
                    .sum::<f32>();

                (phrase.to_string(), score / words.len() as f32)
            })
            .collect::<HashMap<String, f32>>()
    }
}
