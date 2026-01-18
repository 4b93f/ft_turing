use crate::types::Machine;

pub fn validate_machine(machine: &Machine) -> Result<(), String> {
    if !machine.alphabet.contains(&machine.blank) {
        return Err("Blank character must be in alphabet".to_string());
    }
    
    if !machine.alphabet.iter().all(|c| c.len() == 1) {
        return Err("All alphabet characters must be exactly 1 character long".to_string());
    }
    
    if !machine.states.contains(&machine.initial) {
        return Err(format!("Initial state '{}' not found in states list", machine.initial));
    }
    
    if !machine.finals.iter().all(|s| machine.states.contains(s)) {
        return Err("Not all final states are in states list".to_string());
    }
    
    if !machine.transitions.keys().all(|state| machine.states.contains(state)) {
        return Err("Not all transition states are in states list".to_string());
    }
    
    let all_transitions_valid = machine.transitions.values()
        .flat_map(|transitions| transitions.iter())
        .all(|trans| {
            machine.alphabet.contains(&trans.read) &&
            machine.alphabet.contains(&trans.write) &&
            machine.states.contains(&trans.to_state)
        });
    
    if !all_transitions_valid {
        return Err("Some transition characters or target states are invalid".to_string());
    }
    
    Ok(())
}

pub fn validate_input(machine: &Machine, input: &str) -> Result<(), String> {
    if input.contains(&machine.blank) {
        return Err("Input cannot contain blank character".to_string());
    }
    
    if !input.chars().all(|ch| machine.alphabet.contains(&ch.to_string())) {
        return Err("Not all input characters are in alphabet".to_string());
    }
    
    Ok(())
}