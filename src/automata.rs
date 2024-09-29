use std::collections::{HashMap, HashSet};

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
