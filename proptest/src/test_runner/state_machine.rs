//-
// Copyright 2021 The proptest developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Test declaration helpers and runners for abstract state machine testing.

use crate::std_facade::Vec;
use crate::strategy::state_machine::AbstractStateMachine;

/// TODO
pub trait StateMachineTest {
    /// TODO
    type ConcreteState: Clone;
    /// TODO
    type Abstract: AbstractStateMachine;

    /// Initialize the concrete state
    fn init_test(initial_state: <Self::Abstract as AbstractStateMachine>::State) -> Self::ConcreteState;

    /// Apply a transition in the concrete state.
    fn apply_concrete(
        state: Self::ConcreteState,
        transition: &<Self::Abstract as AbstractStateMachine>::Transition,
    ) -> Self::ConcreteState;

    /// Check some invariant on the concrete state after every transition.
    fn invariants(#[allow(unused_variables)] state: &Self::ConcreteState) {}

    /// Run the test sequentially.
    fn test_sequential(
        initial_state: <Self::Abstract as AbstractStateMachine>::State,
        transitions: Vec<<Self::Abstract as AbstractStateMachine>::Transition>,
    ) {
        let mut state = Self::init_test(initial_state);
        for transition in transitions.iter() {
            state = Self::apply_concrete(state, transition);
            Self::invariants(&state);
        }
    }
}

/// TODO
#[macro_export]
macro_rules! prop_state_machine {
    // With proptest config annotation
    (#![proptest_config($config:expr)]
    $(
        $(#[$meta:meta])*
        fn $test_name:ident(sequential $size:expr => $test:ident $(< $( $ty_param:tt ),+ >)?);
    )*) => {
        $(
            proptest! {
                #![proptest_config($config)]
                $(#[$meta])*
                fn $test_name(
                    (initial_state, transitions) in <$test $(< $( $ty_param ),+ >)? as StateMachineTest>::Abstract::sequential_strategy($size)
                ) {
                    $test $(::< $( $ty_param ),+ >)? ::test_sequential(initial_state, transitions)
                }
            }
        )*
    };

    // Without proptest config annotation
    ($(
        $(#[$meta:meta])*
        fn $test_name:ident(sequential $size:expr => $test:ident $(< $( $ty_param:tt ),+ >)?);
    )*) => {
        $(
            proptest! {
                $(#[$meta])*
                fn $test_name(
                    (initial_state, transitions) in <$test $(< $( $ty_param ),+ >)? as StateMachineTest>::Abstract::sequential_strategy($size)
                ) {
                    $test $(::< $( $ty_param ),+ >)? ::test_sequential(initial_state, transitions)
                }
            }
        )*
    };
}
