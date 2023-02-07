// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{Example, ExampleOptions, HashFunction};
use core::marker::PhantomData;
use winterfell::{
    crypto::ElementHasher,
    math::{fields::f64::BaseElement, FieldElement},
    ProofOptions, Prover, StarkProof, Trace, TraceTable, VerifierError,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

mod air;
use air::Mnist;

mod prover;
use prover::MnistProver;

#[cfg(test)]
mod tests;

const TRACE_WIDTH: usize = 1;
type Blake3_192 = winterfell::crypto::hashers::Blake3_192<BaseElement>;
type Blake3_256 = winterfell::crypto::hashers::Blake3_256<BaseElement>;
type Sha3_256 = winterfell::crypto::hashers::Sha3_256<BaseElement>;

// ML INFERENCE EXAMPLE
// ================================================================================================
pub fn get_example(
    options: &ExampleOptions,
) -> Result<Box<dyn Example>, String> {
    let (options, hash_fn) = options.to_proof_options(28, 8);

    match hash_fn {
        HashFunction::Blake3_192 => Ok(Box::new(MnistExample::<Blake3_192>::new(options))),
        HashFunction::Blake3_256 => Ok(Box::new(MnistExample::<Blake3_256>::new(options))),
        HashFunction::Sha3_256 => Ok(Box::new(MnistExample::<Sha3_256>::new(options))),
        _ => Err("The specified hash function cannot be used with this example.".to_string()),
    }
}

pub struct MnistModel {
    pub w1: [[i32; 16]; 40], // 1st weight layer: (40, 16)
    pub w2: [[i32; 10]; 16], // 2nd weight layer: (16, 10)
}

pub struct MnistData {
    pub x: [i32; 40], // (1, 40)
    pub y: i32,      // a correct answer (between 0 and 9)
}

pub struct MnistExample<H: ElementHasher> {
    options: ProofOptions,
    model: MnistModel,
    data: MnistData,
    _hasher: PhantomData<H>,
}

fn read_dim2_data<const C: usize, const R: usize>(filename: &'static str) -> [[i32; C]; R] {
    let file = File::open(filename).expect("shold be able to read file");
    let lines = BufReader::new(file).lines();
    let mut res = [[0 as i32; C]; R];
    for (row, line) in lines.flatten().enumerate() {
        let v: Vec<i32> = line
            .split(',')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        
        for (col, num) in v.iter().enumerate() {
            if row < R && col < C {
                res[row][col] = *num;
            }
        }
    }
    res
}

fn read_dim1_data<const C: usize>(filename: &'static str) -> [i32; C] {
    let file = File::open(filename).expect("shold be able to read file");
    let lines = BufReader::new(file).lines();
    let mut res = [0 as i32; C];
    for (pos, line) in lines.flatten().enumerate() {
        if pos < C {
            res[pos] = line.parse::<i32>().unwrap();
        }
    }
    res
}

impl<H: ElementHasher> MnistExample<H> {
    pub fn new(options: ProofOptions) -> Self {
        let x_data: [[i32; 40]; 10] = read_dim2_data("src/machine_learning/data/x_test.csv");
        let y_data: [i32; 10]= read_dim1_data("src/machine_learning/data/y_test.csv");
        let w1_data: [[i32; 16]; 40] = read_dim2_data("src/machine_learning/data/w1_1d_40_16.csv");
        let w2_data: [[i32; 10]; 16] = read_dim2_data("src/machine_learning/data/w2_1d_16_10.csv");

        let model = MnistModel {
            w1: w1_data,
            w2: w2_data,
        };
        let data = MnistData {
            x: x_data[0],
            y: y_data[0],
        };

        MnistExample {
            options,
            model,
            data,
            _hasher: PhantomData,
        }
    }  
}

impl<H: ElementHasher> Example for MnistExample<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    fn prove(&self) -> StarkProof {
        let prover = MnistProver::<H>::new(self.options.clone());
        let trace = prover.build_trace(&self.model, &self.data);
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        let result = BaseElement::ONE - BaseElement::new(11); // -10
        winterfell::verify::<Mnist, H>(proof, result)
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        let result = BaseElement::ONE;
        winterfell::verify::<Mnist, H>(proof, result)
    }
}
