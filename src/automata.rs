use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq)]
struct HashableHashSet(HashSet<String>);

impl std::ops::Deref for HashableHashSet {
    type Target = HashSet<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Hash for HashableHashSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for item in &self.0 {
            item.hash(state);
        }
    }
}

// Trait for a generic Finite Automaton
pub trait FiniteAutomaton {
    type State;
    type Symbol;

    fn is_accepting(&self, state: &Self::State) -> bool;
    fn transition(&self, state: &Self::State, symbol: &Self::Symbol) -> HashSet<Self::State>;
    fn start_state(&self) -> &Self::State;
}

// Trait for DFA (only one state after transition)
pub trait DFA: FiniteAutomaton {
    fn transition_dfa(&self, state: &Self::State, symbol: &Self::Symbol) -> Option<Self::State>;
}

// Trait for NFA (multiple possible states after transition)
pub trait NFA: FiniteAutomaton {
    fn epsilon_closure(&self, state: &Self::State) -> HashSet<Self::State>;
}

#[derive(Debug)]
pub struct DeterministicFiniteAutomaton {
    pub states: HashSet<String>,
    pub alphabet: HashSet<char>,
    pub transition_function: HashMap<(String, char), String>,
    pub start_state: String,
    pub accept_states: HashSet<String>,
}

impl FiniteAutomaton for DeterministicFiniteAutomaton {
    type State = String;
    type Symbol = char;

    fn is_accepting(&self, state: &Self::State) -> bool {
        self.accept_states.contains(state)
    }

    fn transition(&self, state: &Self::State, symbol: &Self::Symbol) -> HashSet<Self::State> {
        self.transition_dfa(state, symbol)
            .into_iter()
            .collect::<HashSet<_>>()
    }

    fn start_state(&self) -> &Self::State {
        &self.start_state
    }
}

impl DFA for DeterministicFiniteAutomaton {
    fn transition_dfa(&self, state: &Self::State, symbol: &Self::Symbol) -> Option<Self::State> {
        self.transition_function
            .get(&(state.clone(), *symbol))
            .cloned()
    }
}

impl DeterministicFiniteAutomaton {
    pub fn new(
        states: HashSet<String>,
        alphabet: HashSet<char>,
        transition_function: HashMap<(String, char), String>,
        start_state: String,
        accept_states: HashSet<String>,
    ) -> Self {
        Self {
            states,
            alphabet,
            transition_function,
            start_state,
            accept_states,
        }
    }

    pub fn accepts(&self, input: &str) -> bool {
        let mut current_state = self.start_state.clone();

        for symbol in input.chars() {
            if let Some(next_state) = self.transition_dfa(&current_state, &symbol) {
                current_state = next_state;
            } else {
                return false; // Invalid transition
            }
        }

        self.is_accepting(&current_state)
    }
}

#[derive(Debug)]
pub struct NondeterministicFiniteAutomaton {
    pub states: HashSet<String>,
    pub alphabet: HashSet<char>,
    pub transition_function: HashMap<(String, Option<char>), HashSet<String>>,
    pub start_state: String,
    pub accept_states: HashSet<String>,
}

impl FiniteAutomaton for NondeterministicFiniteAutomaton {
    type State = String;
    type Symbol = char;

    fn is_accepting(&self, state: &Self::State) -> bool {
        self.accept_states.contains(state)
    }

    fn transition(&self, state: &Self::State, symbol: &Self::Symbol) -> HashSet<Self::State> {
        self.transition_nfa(state, Some(symbol))
    }

    fn start_state(&self) -> &Self::State {
        &self.start_state
    }
}

impl NFA for NondeterministicFiniteAutomaton {
    fn epsilon_closure(&self, state: &Self::State) -> HashSet<Self::State> {
        let mut closure = HashSet::new();
        closure.insert(state.clone());
        let mut stack = vec![state.clone()];

        while let Some(s) = stack.pop() {
            if let Some(next_states) = self.transition_function.get(&(s.clone(), None)) {
                for next_state in next_states {
                    if closure.insert(next_state.clone()) {
                        stack.push(next_state.clone());
                    }
                }
            }
        }

        closure
    }
}

impl NondeterministicFiniteAutomaton {
    pub fn new(
        states: HashSet<String>,
        alphabet: HashSet<char>,
        transition_function: HashMap<(String, Option<char>), HashSet<String>>,
        start_state: String,
        accept_states: HashSet<String>,
    ) -> Self {
        Self {
            states,
            alphabet,
            transition_function,
            start_state,
            accept_states,
        }
    }

    pub fn transition_nfa(&self, state: &String, symbol: Option<&char>) -> HashSet<String> {
        self.transition_function
            .get(&(state.clone(), symbol.cloned()))
            .cloned()
            .unwrap_or_else(HashSet::new)
    }

    pub fn accepts(&self, input: &str) -> bool {
        let mut current_states = self.epsilon_closure(&self.start_state);

        for symbol in input.chars() {
            let mut next_states = HashSet::new();
            for state in &current_states {
                let reachable = self.transition_nfa(state, Some(&symbol));
                for r in reachable {
                    next_states.extend(self.epsilon_closure(&r));
                }
            }
            current_states = next_states;
        }

        current_states.into_iter().any(|s| self.is_accepting(&s))
    }
}

impl NondeterministicFiniteAutomaton {
    /// Compute the epsilon closure for a set of states
    fn epsilon_closure_set(&self, states: &HashSet<String>) -> HashSet<String> {
        let mut closure = states.clone();
        let mut stack: VecDeque<_> = states.iter().cloned().collect();

        while let Some(state) = stack.pop_front() {
            if let Some(next_states) = self.transition_function.get(&(state.clone(), None)) {
                for next_state in next_states {
                    if closure.insert(next_state.clone()) {
                        stack.push_back(next_state.clone());
                    }
                }
            }
        }

        closure
    }

    /// Compute the transition for a set of NFA states on a given symbol
    fn move_nfa(&self, states: &HashSet<String>, symbol: &char) -> HashSet<String> {
        let mut reachable = HashSet::new();

        for state in states {
            if let Some(next_states) = self
                .transition_function
                .get(&(state.clone(), Some(*symbol)))
            {
                reachable.extend(next_states.clone());
            }
        }

        self.epsilon_closure_set(&reachable)
    }
}

/// Determinize an NFA to a DFA
pub fn determinize(nfa: &NondeterministicFiniteAutomaton) -> DeterministicFiniteAutomaton {
    // Step 1: Initialize states, alphabet, transition function, and accepting states for the DFA
    let mut dfa_states: HashSet<HashableHashSet> = HashSet::new(); // DFA states are sets of NFA states
    let mut dfa_transition_function: HashMap<(HashableHashSet, char), HashSet<String>> =
        HashMap::new();
    let mut dfa_accept_states: HashSet<HashableHashSet> = HashSet::new();

    // Epsilon closure of the start state of NFA is the start state of the DFA
    let start_state: HashableHashSet =
        HashableHashSet(nfa.epsilon_closure_set(&[nfa.start_state.clone()].into_iter().collect()));

    // Queue for BFS exploration of DFA states
    let mut queue: VecDeque<HashableHashSet> = VecDeque::new();
    queue.push_back(start_state.clone());

    // Track visited DFA states
    let mut visited: HashSet<HashableHashSet> = HashSet::new();
    visited.insert(start_state.clone());

    // Step 2: Process each DFA state (set of NFA states)
    while let Some(current_dfa_state) = queue.pop_front() {
        // Check if this DFA state is an accepting state
        if current_dfa_state
            .iter()
            .any(|s| nfa.accept_states.contains(s))
        {
            dfa_accept_states.insert(current_dfa_state.clone());
        }

        // For each symbol in the alphabet, compute the next state
        for symbol in &nfa.alphabet {
            let next_nfa_states = nfa.move_nfa(&current_dfa_state, symbol);

            if !next_nfa_states.is_empty() {
                // Add the new DFA state if it hasn't been visited
                if visited.insert(HashableHashSet(next_nfa_states.clone())) {
                    queue.push_back(HashableHashSet(next_nfa_states.clone()));
                }

                // Record the transition in the DFA's transition function
                dfa_transition_function
                    .insert((current_dfa_state.clone(), *symbol), next_nfa_states);
            }
        }
    }

    // Step 3: Convert the set-based DFA to a DeterministicFiniteAutomaton
    // Map the set-based DFA states to unique string names
    let mut state_map: HashMap<HashableHashSet, String> = HashMap::new();
    let mut state_counter = 0;

    for state_set in visited {
        let state_name = format!("q{}", state_counter);
        state_map.insert(state_set.clone(), state_name);
        state_counter += 1;
    }

    let dfa_start_state = state_map[&start_state].clone();
    let dfa_states: HashSet<String> = state_map.values().cloned().collect();
    let dfa_states = HashableHashSet(dfa_states);
    let dfa_accept_states: HashSet<String> = dfa_accept_states
        .into_iter()
        .map(|s| state_map[&s].clone())
        .collect();
    let dfa_accept_states = HashableHashSet(dfa_accept_states);

    // Convert the transition function to use string-based states
    let dfa_transition_function: HashMap<(String, char), String> = dfa_transition_function
        .into_iter()
        .map(|((from_set, symbol), to_set)| {
            let from_state = state_map[&from_set].clone();
            let to_state = state_map[&HashableHashSet(to_set)].clone();
            ((from_state, symbol), to_state)
        })
        .collect();

    // Return the resulting DFA
    DeterministicFiniteAutomaton {
        states: dfa_states.0,
        alphabet: nfa.alphabet.clone(),
        transition_function: dfa_transition_function,
        start_state: dfa_start_state,
        accept_states: dfa_accept_states.0,
    }
}
