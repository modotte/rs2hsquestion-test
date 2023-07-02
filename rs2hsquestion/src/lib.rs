use std::{
    io::{Stdin, Stdout},
    ops::Deref,
};

use hs_bindgen::*;
use question::{Answer, Question};

enum HSA {
    YES,
    NO,
    RESPONSE(String),
}

#[hs_bindgen]
fn ask_confirmation(question_to_ask: &str) -> String {
    match Question::new(question_to_ask).confirm() {
        Answer::RESPONSE(s) => s,
        Answer::YES => "yes".to_string(),
        Answer::NO => "no".to_string(),
    }
}
