use Rust_Learning::DeterministicFiniteAutomaton;
use Rust_Learning::NondeterministicFiniteAutomaton;

fn main() {
    // DFA example
    let dfa = DeterministicFiniteAutomaton::new(
        vec!["q0".to_string(), "q1".to_string()]
            .into_iter()
            .collect(),
        vec!['a', 'b'].into_iter().collect(),
        vec![
            (("q0".to_string(), 'a'), "q1".to_string()),
            (("q1".to_string(), 'b'), "q0".to_string()),
        ]
        .into_iter()
        .collect(),
        "q0".to_string(),
        vec!["q1".to_string()].into_iter().collect(),
    );

    println!("DFA accepts 'ab': {}", dfa.accepts("ab")); // true
    println!("DFA accepts 'aa': {}", dfa.accepts("aa")); // false

    // NFA example
    let nfa = NondeterministicFiniteAutomaton::new(
        vec!["q0".to_string(), "q1".to_string(), "q2".to_string()]
            .into_iter()
            .collect(),
        vec!['a', 'b'].into_iter().collect(),
        vec![
            (
                ("q0".to_string(), Some('a')),
                vec!["q0".to_string(), "q1".to_string()]
                    .into_iter()
                    .collect(),
            ),
            (
                ("q1".to_string(), Some('b')),
                vec!["q2".to_string()].into_iter().collect(),
            ),
            (
                ("q2".to_string(), None),
                vec!["q0".to_string()].into_iter().collect(),
            ), // Epsilon transition
        ]
        .into_iter()
        .collect(),
        "q0".to_string(),
        vec!["q2".to_string()].into_iter().collect(),
    );

    println!("NFA accepts 'ab': {}", nfa.accepts("ab")); // true
    println!("NFA accepts 'aa': {}", nfa.accepts("aa")); // false
}
