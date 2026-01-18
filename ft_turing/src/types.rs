use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
	Left,
	Right,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Transition {
    pub read: String,
    pub to_state: String,
    pub write: String,
    pub action: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Machine {
	pub name: String,
	pub alphabet: Vec<String>,
	pub blank: String,
	pub states: Vec<String>,
	pub initial: String,
	pub finals: Vec<String>,
	pub transitions: HashMap<String, Vec<Transition>>,
}

#[derive(Debug, Clone)]
pub struct Tape {
    pub cells: HashMap<i64, char>,
    pub blank: char,
    pub head_position: i64,
}