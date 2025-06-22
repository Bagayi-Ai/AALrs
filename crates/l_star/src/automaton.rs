use std::collections::{HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;


#[derive(Clone, Debug)]
pub struct DfaState<StateId, TransitionLabel> {
    state_id: StateId,
    is_accepting: bool,
    pub transitions: HashMap<TransitionLabel, StateId>,
}

impl<StateId, TransitionLabel> DfaState<StateId, TransitionLabel> {
    pub fn new(state_id: StateId, is_accepting: bool) -> Self {
        DfaState {
            state_id,
            is_accepting,
            transitions: HashMap::new(),
        }
    }

    pub fn get_state_id(&self) -> &StateId {
        &self.state_id
    }

    pub fn is_accepting(&self) -> bool {
        self.is_accepting
    }

    pub fn set_accepting(&mut self, is_accepting: bool) {
        self.is_accepting = is_accepting;
    }
}


impl<StateId, T, TransitionLabel> DfaState<StateId, TransitionLabel>
where
    StateId: IntoIterator<Item = T> + Clone,
    T: Display,
{
    pub fn serialize_state_id(&self, sep: &str) -> String {
        self.state_id
            .clone()
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(sep)
    }
}



#[derive(Clone, Debug)]
pub struct Automaton<StateId: Eq + Hash + Clone + Debug, TransitionLabel: Eq + Hash + Clone + Default> {
    states: HashMap<StateId, DfaState<StateId, TransitionLabel>>,
    initial_state: StateId,
}

impl <T: Eq + Hash + Clone + Debug + Display, StateId: Eq + Hash + Clone + Debug + IntoIterator<Item = T>, TransitionLabel: Eq + Hash + Clone + Debug + Default> Automaton<StateId, TransitionLabel> {

    pub fn new(initial_state: DfaState<StateId, TransitionLabel>) -> Self {
        let mut states: HashMap<StateId, DfaState<StateId, TransitionLabel>> = HashMap::new();
        states.insert(initial_state.state_id.clone(), initial_state.clone());
        
        Automaton {
            states,
            initial_state: initial_state.state_id,
        }
    }

    pub fn get_states(&self) -> &HashMap<StateId, DfaState<StateId, TransitionLabel>> {
        &self.states
    }

    pub fn add_transition(&mut self, from: &DfaState<StateId, TransitionLabel>, to: &DfaState<StateId, TransitionLabel>, transition_label: &TransitionLabel) {
        // Ensure both states exist
        if !self.states.contains_key(&from.state_id) {
            self.states.insert(from.state_id.clone(), from.clone());
        }
        if !self.states.contains_key(&to.state_id) {
            self.states.insert(to.state_id.clone(), to.clone());
        }

        if let Some(from_state) = self.states.get_mut(&from.state_id) {
                from_state.transitions.insert(transition_label.clone(), to.state_id.clone());
        }
    }

    pub fn add_state(&mut self, state: DfaState<StateId, TransitionLabel>) {
        if !self.states.contains_key(&state.state_id) {
            self.states.insert(state.state_id.clone(), state.clone());
        }
    }

    pub fn get_state(&self, state_id: &StateId) -> Option<&DfaState<StateId, TransitionLabel>> {
        self.states.get(state_id)
    }

    pub fn get_initial_state(&self) -> Option<&DfaState<StateId, TransitionLabel>> {
        self.get_state(&self.initial_state)
    }

    pub fn set_initial_state(&mut self, state: &DfaState<StateId, TransitionLabel>) {
        self.initial_state = state.state_id.clone();
    }

    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph DFA {\n");

        // Mark accepting states
        for (state_id, state) in &self.states {
            let state_id_str = state.serialize_state_id("");
            if state.is_accepting() {
                dot.push_str(&format!("    {:?} [shape=doublecircle];\n", &state_id_str));
            } else {
                dot.push_str(&format!("    {:?};\n", &state_id_str));
            }
        }


        let initial_state = self.get_initial_state().unwrap();
        // Initial state arrow
        dot.push_str(&format!("    __start__ [shape=point];\n    __start__ -> {:?} [label = {:?}];\n", initial_state.serialize_state_id(""), TransitionLabel::default()));

        // Transitions
        for (state_id, state) in &self.states {
            for (label, target) in &state.transitions {
                let target_state = self.states.get(target).unwrap();
                dot.push_str(&format!(
                    "    {:?} -> {:?} [label = {:?}];\n",
                    &state.serialize_state_id(""), target_state.serialize_state_id(""), label
                ));
            }
        }

        dot.push_str("}\n");
        dot
    }
}
