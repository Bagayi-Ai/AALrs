use regex::Regex;
use crate::teacher::Teacher;
use crate::automaton::Automaton;
use std::collections::HashSet;

pub struct RegexTeacher {
    regex: Regex,
}

impl RegexTeacher {
    pub fn new(regex: String) -> Self {

        RegexTeacher { 
            regex: Regex::new(&regex).expect("Invalid regex pattern"),
        }
    }
}


impl Teacher<String> for RegexTeacher {

    fn membership_query(&self, states: Vec<String>) -> bool {
        let input = states.join("");
        self.regex.is_match(&input)
    }

    fn validate_hypothesis(&self, automaton: Automaton<Vec<String>, String>) -> Result<bool, HashSet<Vec<String>>> {
        Ok(true)
    }
}