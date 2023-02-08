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

mod air;
use air::Relu;

mod prover;
use prover::ReluProver;

#[cfg(test)]
mod tests;

const TRACE_WIDTH: usize = 1;
type Blake3_192 = winterfell::crypto::hashers::Blake3_192<BaseElement>;
type Blake3_256 = winterfell::crypto::hashers::Blake3_256<BaseElement>;
type Sha3_256 = winterfell::crypto::hashers::Sha3_256<BaseElement>;

// Utils
pub fn from_signed_int_field(x: i32) -> BaseElement {
    if x < 0 {
        let p: u64 = (x * -1) as u64;
        BaseElement::ZERO - BaseElement::new(p)
    } else {
        BaseElement::new(x as u64)
    }
}

// ReLU example
// ================================================================================================
pub fn get_example(
    options: &ExampleOptions,
    input: i32,
) -> Result<Box<dyn Example>, String> {
    let (options, hash_fn) = options.to_proof_options(28, 8);

    match hash_fn {
        HashFunction::Blake3_192 => Ok(Box::new(ReluExample::<Blake3_192>::new(options, input))),
        HashFunction::Blake3_256 => Ok(Box::new(ReluExample::<Blake3_256>::new(options, input))),
        HashFunction::Sha3_256 => Ok(Box::new(ReluExample::<Sha3_256>::new(options, input))),
        _ => Err("The specified hash function cannot be used with this example.".to_string()),
    }
}

pub struct ReluExample<H: ElementHasher> {
    options: ProofOptions,
    input: i32,
    output: BaseElement,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> ReluExample<H> {
    pub fn new(options: ProofOptions, input: i32) -> Self {
        let mut result: u64 = 0;
        if input >= 0 {
            result = input as u64;
        }

        ReluExample {
            options,
            input,
            output: BaseElement::new(result),
            _hasher: PhantomData,
        }
    }
}

impl<H: ElementHasher> Example for ReluExample<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    fn prove(&self) -> StarkProof {
        let prover = ReluProver::<H>::new(self.options.clone());
        let trace = prover.build_trace(self.input);
        prover.prove(trace).unwrap()
    }

    fn verify(&self, proof: StarkProof) -> Result<(), VerifierError> {
        let a = from_signed_int_field(self.input);
        let result = a + BaseElement::new(7);
        winterfell::verify::<Relu, H>(proof, result)
    }

    fn verify_with_wrong_inputs(&self, proof: StarkProof) -> Result<(), VerifierError> {
        let result = BaseElement::ZERO;
        winterfell::verify::<Relu, H>(proof, result)
    }
}
