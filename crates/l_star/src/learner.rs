use std::{collections::{HashMap, HashSet}, fmt::{Debug, Display}, hash::Hash, iter::once, vec};
use crate::teacher::Teacher;
use crate::automaton::{Automaton, DfaState};


type ObsKe<T> = Vec<T>;


#[derive(Debug)]
struct ObservationTable<T: Eq + Hash + Default + Clone> {
    alphabets: HashSet<T>,
    s_prefixes : HashSet<ObsKe<T>>,
    e_suffixes: HashSet<ObsKe<T>>,
    table: HashMap<ObsKe<T>, HashMap<ObsKe<T>, bool>>,
}

impl <T: Eq + Hash + Default + Clone> ObservationTable<T> {

    pub fn new(alphabets: HashSet<T>) -> Self {

        let e_suffixes: HashSet<ObsKe<T>> = once(vec![T::default()])
            .chain(alphabets.iter().map(|a| vec![a.clone()]))
            .collect();

        ObservationTable {
            alphabets,
            s_prefixes: HashSet::from_iter(once(vec![T::default()])), // Start with the default element
            e_suffixes, // Start with the default element
            table: HashMap::new(),
        }
    }

    fn is_consistent(&self) -> Result<bool, Option<(ObsKe<T>, ObsKe<T>, T)>> {
        /*
        An observation table is called consistent provided that
        whenever s1 and s2 are elements of S such that row(s,) = row(s,),
        for all a inA,row(s, .a)=row(s, .a).
            If (S, E, T) is a closed,
         */
        for s1 in &self.s_prefixes {
            if let Some(s1_row) = self.table.get(s1) { 
                for s2 in &self.s_prefixes {
                    // let s2_row = self.table.get(s2);
                    if let Some(s2_row) = self.table.get(s2) {
                        if s1_row == s2_row {
                            for a in &self.alphabets {
                                if let (Some(s1_a_row), Some(s2_a_row)) = (
                                    self.table.get(&concat_vec_elem(s1, a)),
                                    self.table.get(&concat_vec_elem(s2, a)))
                                    {
                                        if s1_a_row != s2_a_row {
                                            return Err(Some((s1.clone(), s2.clone(), a.clone())));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        Ok(true)
    }

    fn is_closed(&self) -> Result<bool, (ObsKe<T>)>{
        /*
        An observation table is called closed provided that for each t in S. A there exists an s in S such that
        row(t) = row(s).
         */

        for t in self.get_sa() {
            if let Some(t_row) = self.table.get(&t){
                let mut found = false;
                for s in &self.s_prefixes {
                    if let Some(s_row) = self.table.get(s){
                        if t_row == s_row {
                            found = true;
                            break;
                        }
                    }
                }

                if !found {
                    // If no such s exists, return the first t and the first a from t
                    return Err(t);
                }
            }
        }
        Ok(true)
    }

    fn get_sa(&self) -> HashSet<ObsKe<T>> {
        let mut sa: HashSet<ObsKe<T>> = HashSet::new();
        for s in &self.s_prefixes {
            for a in &self.alphabets {
                sa.insert(s.iter().cloned().chain(once(a.clone())).collect()); // Combine s and a into a single vector
            }
        }
        sa
    }

    fn get_rows(&self) -> HashSet<Vec<T>> {
        let mut rows: HashSet<Vec<T>> = HashSet::new();

        // Add all S (prefixes)
        for s in &self.s_prefixes {
            rows.insert(s.clone());
        }

        for sa in self.get_sa() {
            rows.insert(sa); // Add all S·A (prefix + symbol)
        }
        rows
    }

    fn get_columns(&self) -> HashSet<Vec<T>> {
        self.e_suffixes.clone()
    }

    fn update(&mut self, row: &ObsKe<T>, col: &ObsKe<T>, value: bool) {
        self.table.entry(row.clone())
            .or_default()
            .insert(col.clone(), value);
    }

}

fn concat_vec_elem<T: Clone>(a: &[T], b: &T) -> Vec<T> {
    a.iter().cloned().chain(once(b.clone())).collect()
}

fn concat_vecs<T: Clone>(a: &[T], b: &[T]) -> Vec<T> {
    a.iter().cloned().chain(b.iter().cloned()).collect()
}


pub struct Learner<T: Eq + Hash + Clone + Default> {
    observation_table: ObservationTable<T>,
    teacher: Box<dyn Teacher<T>>,
}


impl<T: Eq + Hash + Clone + Default + Debug + Display> Learner<T> {
    pub fn new(alphabets: HashSet<T>, teacher: Box<dyn Teacher<T>>) -> Self {

        Learner {
            observation_table: ObservationTable::new(alphabets),
            teacher,
        }
    }


    fn update_observation_table(&mut self){
        let rows = self.observation_table.get_rows();
        let columns = self.observation_table.get_columns();

        for row in rows {
            for col in &columns {
                let c: Vec<_> = concat_vecs(&row, &col);
                // Query the teacher for information about the (row, col) pair
                let response = self.teacher.membership_query(c);
                // Update the observation table with the teacher's response
                self.observation_table.update(&row, &col, response);
            }
        }
    }


    fn gen_hypothesis(&self) -> Automaton<ObsKe<T>, T> {
        /*
        a corresponding acceptor M(S, E, T) over the alphabet A, with state set Q, initial state qO, accepting states F, and transition function 6 as follows:
            Q= {row(s):sES}, 
            q0= row(L),
            F= {row(s):s ∈ S and T(s)=1}, 
            
            δ(row(s), a) = row(s .a).
         */

        let mut automaton: Automaton<ObsKe<T>, T> = Automaton::new(DfaState::new(vec![T::default()], false));

        let mut state_distinguish: HashMap<bool, ObsKe<T>> = HashMap::new();

        for prefix in &self.observation_table.s_prefixes {
            let state_id = prefix.clone();

            let mut state: DfaState<ObsKe<T>, T>  = DfaState::new(state_id, false);

            if let Some(true) = self.observation_table.table.get(&prefix.clone()).and_then(|t| t.get(&vec![T::default()])) {
                // Check if the state is accepting based on the table
                state.set_accepting(true);
            }

            automaton.add_state(state.clone());

            if let Some(t_value) = self.observation_table.table.get(&prefix.clone()).and_then(|t| t.get(&vec![T::default()])) {
                state_distinguish.insert(t_value.clone(), prefix.clone());
            }

            if prefix.clone() == vec![T::default()] {
                automaton.set_initial_state(&state);
            }
        }

        for prefix in &self.observation_table.s_prefixes {
            let state_s = automaton.get_state(&prefix.clone()).unwrap().clone();
            for a in &self.observation_table.alphabets {
                let state_sa_id = concat_vec_elem(prefix, a);

                if let Some(t_value) = self.observation_table.table.get(&state_sa_id).and_then(|t| t.get(&vec![T::default()])) {
                    if let Some(state) = state_distinguish.get(t_value) {
                        let target_state = automaton.get_state(state).unwrap().clone();
                        automaton.add_transition(&state_s, &target_state, a);
                    }
                }
            }
        }
        automaton
    }

    pub fn learn(&mut self) -> Automaton<ObsKe<T>, T> {
        loop {
            self.update_observation_table();
            loop {
                let is_consistent = self.observation_table.is_consistent();

                if let Err(Some((s1, s2, a ))) = is_consistent.clone() {
                    // find e that resulted in inconsistency
                    let s1_a_row = self.observation_table.table.get(&concat_vec_elem(&s1, &a));
                    let s2_a_row = self.observation_table.table.get(&concat_vec_elem(&s2, &a));

                    if let (Some(s1_a_row), Some(s2_a_row)) = (s1_a_row, s2_a_row) {
                        for (e, v) in s1_a_row.iter() {
                            if let Some(s2_a_v) = s2_a_row.get(e) {
                                if v != s2_a_v {
                                    self.observation_table.e_suffixes.insert(e.clone());
                                }
                            }
                        }
                    }
                    self.update_observation_table();
                }

                let is_closed = self.observation_table.is_closed();

                if let Err(sa) = is_closed.clone() {
                    self.observation_table.s_prefixes.insert(sa);
                    self.update_observation_table();
                }

                if matches!(is_closed, Ok(true)) && matches!(is_consistent, Ok(true)) {
                    break; // Exit the loop if the table is closed or consistent
                }
            }

            let hypothesis = self.gen_hypothesis();

            match self.teacher.validate_hypothesis(hypothesis.clone()) {
                Ok(true) => {
                    println!("Learning completed successfully.");
                    return hypothesis; // Learning is complete
                },
                Err(counterexample) => {
                    // If a counterexample was provided, we need to update the observation table
                    for e in counterexample {
                        self.observation_table.s_prefixes.extend(vec![e.clone()]);
                    }
                }
                _ => {
                    // If no counterexample was provided, we can continue learning
                    panic!("Unexpected response from teacher");
                }
            }
        }
    }
}
