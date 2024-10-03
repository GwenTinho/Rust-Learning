use Rust_Learning::{DeterministicFiniteAutomaton, NondeterministicFiniteAutomaton};
fn main() {
    // Create your DFA or NFA and visualize it
    let dfa = DeterministicFiniteAutomaton {
        states: vec!["q0".to_string(), "q1".to_string()]
            .into_iter()
            .collect(),
        alphabet: vec!['a', 'b'].into_iter().collect(),
        transition_function: vec![
            (("q0".to_string(), 'a'), "q1".to_string()),
            (("q1".to_string(), 'b'), "q0".to_string()),
        ]
        .into_iter()
        .collect(),
        start_state: "q0".to_string(),
        accept_states: vec!["q1".to_string()].into_iter().collect(),
    };

    // Similarly, create and display an NFA
}
