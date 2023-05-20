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

pub struct WeightParams {
    w_tf: f32,
    w_c: f32,
    w_pf: f32,
    w_pl: f32,
    w_avg: f32,
}

impl WeightParams {
    pub fn statistical_default() -> Self {
        let base_weight = 0.2_f32;
        Self {
            w_tf: base_weight,
            w_c: base_weight,
            w_pf: base_weight,
            w_pl: base_weight,
            w_avg: base_weight,
        }
    }

    pub fn main_default() -> Self {
        let base_weight = 1.0_f32;
        Self {
            w_tf: base_weight,
            w_c: base_weight,
            w_pf: base_weight,
            w_pl: base_weight,
            w_avg: base_weight,
        }
    }

    pub fn new(w_tf: f32, w_c: f32, w_pf: f32, w_pl: f32, w_avg: f32) -> Self {
        Self {
            w_tf,
            w_c,
            w_pf,
            w_pl,
            w_avg,
        }
    }

    pub fn get_weights(&self) -> (f32, f32, f32, f32, f32) {
        (
            Self::check_weight_renge(self.w_tf),
            Self::check_weight_renge(self.w_c),
            Self::check_weight_renge(self.w_pf),
            Self::check_weight_renge(self.w_pl),
            Self::check_weight_renge(self.w_avg),
        )
    }

    fn check_weight_renge(weight: f32) -> f32 {
        if weight > 0.0 && weight <= 1.0 {
            weight
        } else {
            0.2
        }
    }
}

pub enum YakeParams<'a> {
    WithDefaults(&'a str, &'a [String]),
    BaseParams(&'a str, &'a [String], usize, usize, f32),
    All(&'a str, &'a [String], usize, usize, f32, WeightParams),
}

type Candidate<'a> = &'a str;
type StopWord<'a> = &'a [String];
type NgramSize = usize;
type WindowSize = usize;
type Threshold = f32;
type Weights = (f32, f32, f32, f32, f32);

impl<'a> YakeParams<'a> {
    pub fn get_values(
        &self,
    ) -> (
        Candidate<'a>,
        StopWord<'a>,
        NgramSize,
        WindowSize,
        Threshold,
        Weights,
    ) {
        match self {
            YakeParams::WithDefaults(text, stop_words) => (
                text,
                stop_words,
                3,
                3,
                0.8,
                WeightParams::main_default().get_weights(),
            ),
            YakeParams::BaseParams(text, stop_words, n_gram_size, window_size, threshold) => (
                text,
                stop_words,
                *n_gram_size,
                *window_size,
                *threshold,
                WeightParams::statistical_default().get_weights(),
            ),
            YakeParams::All(
                text,
                stop_words,
                n_gram_size,
                window_size,
                threshold,
                weight_params,
            ) => (
                text,
                stop_words,
                *n_gram_size,
                *window_size,
                *threshold,
                weight_params.get_weights(),
            ),
        }
    }
}
