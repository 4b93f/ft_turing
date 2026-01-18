use crate::types::Tape;
use std::collections::HashMap;

impl Tape {
    pub fn new(input: &str, blank: char) -> Tape {
        let cells: HashMap<i64, char> = input
            .chars()
            .enumerate()  // [(0, '1'), (1, '1'), (2, '1'), (3, '+'), (4, '1'), (5, '1')]
            .map(|(i, c)| (i as i64, c)) 
            .collect(); // return hashMap
        
        Tape {
            cells,
            blank,
            head_position: 0,
        }
    }
    
    pub fn read(&self) -> char {
        *self.cells.get(&self.head_position).unwrap_or(&self.blank)
    }
    
    pub fn write(&self, symbol: char) -> Tape {
        let mut new_cells = self.cells.clone(); // because cells is immutable 
        
        if symbol == self.blank {
            new_cells.remove(&self.head_position);
        } else {
            new_cells.insert(self.head_position, symbol);
        }
        
        Tape {
            cells: new_cells,
            blank: self.blank,
            head_position: self.head_position,
        }
    }
    
    pub fn move_head(&self, direction: &str) -> Tape {
        let new_position = match direction {
            "LEFT" => self.head_position - 1,
            "RIGHT" => self.head_position + 1,
            _ => self.head_position,
        };
        
        Tape {
            cells: self.cells.clone(),
            blank: self.blank,
            head_position: new_position,
        }
    }
    
    pub fn to_string(&self) -> String {
        if self.cells.is_empty() {
            return format!("[<{}>]", self.blank); // should not happen
        }
        
        let cells_positions: Vec<i64> = self.cells.keys().copied().collect();
        let min_pos = *cells_positions.iter().min().unwrap_or(&0);
        let max_pos = *cells_positions.iter().max().unwrap_or(&0);
        
        let start = min_pos.min(self.head_position - 2);
        let end = max_pos.max(self.head_position + 10);
        
        let chars: Vec<String> = (start..=end)
            .map(|pos| {
                let characters = *self.cells.get(&pos).unwrap_or(&self.blank);
                if pos == self.head_position {
                    format!("<{}>", characters)
                } else {
                    characters.to_string()
                }
            })
            .collect(); // return vec
        
        format!("[{}]", chars.join(""))
    }
}