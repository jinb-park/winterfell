// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{
    air::Relu, BaseElement, ElementHasher, FieldElement, PhantomData, ProofOptions, Prover,
    Trace, TraceTable, TRACE_WIDTH, from_signed_int_field
};

pub struct ReluProver<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> ReluProver<H> {
    pub fn new(options: ProofOptions) -> Self {
        Self {
            options,
            _hasher: PhantomData,
        }
    }

    pub fn build_trace(&self, input: i32) -> TraceTable<BaseElement> {
        let len = 8;

        let mut trace = TraceTable::new(TRACE_WIDTH, len);
        let mut state = vec![BaseElement::ZERO; TRACE_WIDTH];

        state[0] = from_signed_int_field(input);
        trace.update_row(0, &state);

        for i in 1..len {
            state[0] = state[0] + BaseElement::ONE;
            trace.update_row(i, &state);
        }

        trace
    }
}

impl<H: ElementHasher> Prover for ReluProver<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    type BaseField = BaseElement;
    type Air = Relu;
    type Trace = TraceTable<BaseElement>;
    type HashFn = H;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        let last_step = trace.length() - 1;
        trace.get(0, last_step)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
