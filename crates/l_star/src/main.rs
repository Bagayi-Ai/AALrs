use std::collections::HashSet;
use std::fs::File;
use std::io::Write;

use l_star::teachers::regex_teacher::{RegexTeacher};
use l_star::learner::Learner;


fn main() {

    let regex_teacher = RegexTeacher::new(
        "^(b*ab*){1}(b*ab*b*ab*){0,}$".to_string());

    let mut learner = Learner::new(
        HashSet::from(["a".to_string(), "b".to_string()]), 
        Box::new(regex_teacher));

    let learnt_hypothesis = learner.learn();

    let mut file = File::create("hypothesis.dot").expect("Unable to create file");
    file.write_all(learnt_hypothesis.to_dot().as_bytes())
        .expect("Unable to write to file");
    println!("Hypothesis written to hypothesis.dot");

}
