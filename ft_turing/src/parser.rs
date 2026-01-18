use crate::types::Machine;
use std::fs;
use std::error::Error;

pub fn parse_machine_from_file(file_path: &str) -> Result<Machine, Box<dyn Error>> {
	let file_content = fs::read_to_string(file_path)?; // read content
	let machine: Machine = serde_json::from_str(&file_content)?; // parse Json
	Ok(machine)
}