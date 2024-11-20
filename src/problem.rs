use argmin::{core::CostFunction, solver::simulatedannealing::Anneal};
use rand::{thread_rng, Rng};

use crate::keyboard::{index_pair, InputKey, IN_KEYS_COUNT, OUT_KEYS_COUNT, OUT_KEY_PAIR_PROBS};

// Only the first OUT_KEYS_COUNT are actually used for the cost
type State = [InputKey; IN_KEYS_COUNT];

pub struct Problem;

impl CostFunction for Problem {
    type Param = State;

    type Output = f64;

    // Could get some crazy speed up if cached this then only update the parts changed in Anneal (like order of magnitude)
    fn cost(&self, param: &Self::Param) -> Result<Self::Output, argmin::core::Error> {
        let mut cost: f64 = 0.0;
        for prev_index in 0..OUT_KEYS_COUNT {
            let prev = &param[prev_index];
            for curr_index in 0..OUT_KEYS_COUNT {
                cost += param[curr_index].get_cost(prev) * OUT_KEY_PAIR_PROBS[index_pair(prev_index, curr_index)]
            }
        }

        Ok(cost)
    }
}

impl Anneal for Problem {
    type Param = State;

    type Output = State;

    type Float = f64;

    fn anneal(&self, param: &Self::Param, extent: Self::Float) -> Result<Self::Output, argmin::core::Error> {
        let mut out: State = *param;

        let mut rng = thread_rng();
        // Lazy ceilling
        for _i in 0..((extent + 1.0) as u64) {
            out.swap(rng.gen_range(0..IN_KEYS_COUNT), rng.gen_range(0..IN_KEYS_COUNT));
        }

        Ok(out)
    }
}

