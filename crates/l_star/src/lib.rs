/*
An implementation of Angluin's L* algorithm for learning DFAs, as described in the paper "Learning Regular Sets from
 * Queries and Counterexamples".
https://people.eecs.berkeley.edu/~dawnsong/teaching/s10/papers/angluin87.pdf
 */

pub mod learner;
mod automaton;
mod teacher;

pub mod teachers{
    pub mod regex_teacher;
}