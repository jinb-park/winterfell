// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use super::{BaseElement, FieldElement, ProofOptions, TRACE_WIDTH};
use crate::utils::are_equal;
use winterfell::{
    Air, AirContext, Assertion, EvaluationFrame, TraceInfo, TransitionConstraintDegree,
};

pub struct Mnist {
    context: AirContext<BaseElement>,
    result: BaseElement,
}

impl Air for Mnist {
    type BaseField = BaseElement;
    type PublicInputs = BaseElement;

    fn new(trace_info: TraceInfo, pub_inputs: Self::BaseField, options: ProofOptions) -> Self {
        // [Q] how to set proper constraint degrees? -> https://github.com/jinb-park/winterfell/tree/main/air
        let degrees = vec![
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(2),
            TransitionConstraintDegree::new(2),
        ];
        assert_eq!(TRACE_WIDTH, trace_info.width());
        Mnist {
            context: AirContext::new(trace_info, degrees, 1, options),
            result: pub_inputs,
        }
    }

    fn context(&self) -> &AirContext<Self::BaseField> {
        &self.context
    }

    fn evaluate_transition<E: FieldElement + From<Self::BaseField>>(
        &self,
        frame: &EvaluationFrame<E>,
        _periodic_values: &[E],
        result: &mut [E],
    ) {
        let current = frame.current();
        let next = frame.next();
        
        debug_assert_eq!(TRACE_WIDTH, current.len());
        debug_assert_eq!(TRACE_WIDTH, next.len());

        result[0] = are_equal(next[0], current[0] * current[1]);
        result[1] = are_equal(next[1], current[0] * current[1]);
        result[2] = are_equal(next[2], next[1] * current[2]);

        //result[0] = are_equal(next[0], current[0] - E::ONE);
    }

    /*
    fn get_periodic_column_values(&self) -> Vec<Vec<Self::BaseField>> {
        let v = Vec::new();

        let v1 = Vec::new();
        v1.push(Self::BaseField::new(2));

        v.push(Self::BaseField::new(2));
        v.push(Self::BaseField::new(3));
        v.push(Self::BaseField::new(4));
        v.push(Self::BaseField::new(1));
        v.push(Self::BaseField::new(1));
        v.push(Self::BaseField::new(1));
        v.push(Self::BaseField::new(1));
        v.push(Self::BaseField::new(1));

        Vec::new(v)
    } */

    fn get_assertions(&self) -> Vec<Assertion<Self::BaseField>> {
        let last_step = self.trace_length() - 1;
        vec![
            //Assertion::single(0, 0, Self::BaseField::ONE - Self::BaseField::new(4)),
            //Assertion::single(3, last_step, self.result),
            Assertion::single(2, 1, Self::BaseField::new(24)),
        ]
    }
}
