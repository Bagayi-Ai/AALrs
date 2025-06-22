use std::collections::HashSet;
use std::hash::Hash;
use std::fmt::Debug;

use crate::automaton::Automaton;

pub trait Teacher<T: Eq + Hash + Clone + Debug + Default> {

    fn membership_query(&self, states: Vec<T>) -> bool;

    fn validate_hypothesis(&self, automaton: Automaton<Vec<T>, T>) -> Result<bool, HashSet<Vec<T>>>;
}