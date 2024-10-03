use std::collections::{HashMap, HashSet};

use crate::DeterministicFiniteAutomaton;

struct Sample {
    positive: HashSet<String>,
    negative: HashSet<String>,
}

fn build_pta(s: Sample) -> DeterministicFiniteAutomaton {
    // Step 1: Initialize states, alphabet, transition function, and accepting states
    let mut states: HashSet<String> = HashSet::new();
    let mut alphabet: HashSet<char> = HashSet::new();
    let mut transition_function: HashMap<(String, char), String> = HashMap::new();
    let mut accept_states: HashSet<String> = HashSet::new();

    // Define the start state
    let start_state = "q0".to_string();
    states.insert(start_state.clone());

    let mut next_state_index = 1; // Counter to generate unique state names

    // Step 2: Process positive samples to build the PTA
    for word in s.positive {
        let mut current_state = start_state.clone();

        // Process each character in the string
        for symbol in word.chars() {
            // Update the alphabet set
            alphabet.insert(symbol);

            // Check if there is an existing transition
            if let Some(next_state) = transition_function.get(&(current_state.clone(), symbol)) {
                // Move to the next state if transition exists
                current_state = next_state.clone();
            } else {
                // Create a new state
                let next_state = format!("q{}", next_state_index);
                next_state_index += 1;

                // Insert the new state and transition
                states.insert(next_state.clone());
                transition_function.insert((current_state.clone(), symbol), next_state.clone());

                // Move to the new state
                current_state = next_state;
            }
        }

        // Mark the final state of this string as an accepting state
        accept_states.insert(current_state);
    }

    // Step 3: Create and return the DFA
    DeterministicFiniteAutomaton {
        states,
        alphabet,
        transition_function,
        start_state,
        accept_states,
    }
}

// fn rpni(s: Sample) -> DeterministicFiniteAutomaton {}
