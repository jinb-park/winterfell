// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{
    air::Mnist, BaseElement, ElementHasher, FieldElement, PhantomData, ProofOptions, Prover,
    Trace, TraceTable, TRACE_WIDTH, MnistModel, MnistData, from_signed_int_field
};

pub struct MnistProver<H: ElementHasher> {
    options: ProofOptions,
    _hasher: PhantomData<H>,
}

impl<H: ElementHasher> MnistProver<H> {
    pub fn new(options: ProofOptions) -> Self {
        Self {
            options,
            _hasher: PhantomData,
        }
    }

    pub fn build_trace(&self, model: &MnistModel, data: &MnistData) -> TraceTable<BaseElement> {
        let len = 8;

        let mut trace = TraceTable::new(TRACE_WIDTH, len);
        let mut state = vec![BaseElement::ZERO; TRACE_WIDTH];

        // [0] a: (-2,4), b: (3,-7)^T --> expected sum: -2*4 + 3*-7
        // [1~7] (0,0), (0,0) --> dummy --> expected sum: 0

        // a: (2*2) -> (1,2,3,4), b: (2*2) -> (5,6,7,8)T 
        // a*b = c
        let a: (i64, i64, i64, i64) = (1, 2, 3, 4);
        let b: (i64, i64, i64, i64) = (5, 6, 7, 8);

        let a2 = (from_signed_int_field(a.0), from_signed_int_field(a.1), from_signed_int_field(a.2), from_signed_int_field(a.3));
        let b2 = (from_signed_int_field(b.0), from_signed_int_field(b.1), from_signed_int_field(b.2), from_signed_int_field(b.3));

        state[0] = BaseElement::new(2);
        state[1] = BaseElement::new(3);
        state[2] = BaseElement::new(4);
        trace.update_row(0, &state);

        for i in 1..len {
            state[0] = state[0] * state[1];
            state[1] = state[0];
            state[2] = state[2] * state[1];

            trace.update_row(i, &state);
        }

        trace
    }
}

impl<H: ElementHasher> Prover for MnistProver<H>
where
    H: ElementHasher<BaseField = BaseElement>,
{
    type BaseField = BaseElement;
    type Air = Mnist;
    type Trace = TraceTable<BaseElement>;
    type HashFn = H;

    fn get_pub_inputs(&self, trace: &Self::Trace) -> BaseElement {
        let last_step = trace.length() - 1;
        trace.get(2, 1)
    }

    fn options(&self) -> &ProofOptions {
        &self.options
    }
}
