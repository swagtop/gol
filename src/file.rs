use std::fs;
use crate::state::Cell;

// Character order from: https://stackoverflow.com/a/74186686
const ORDER: &str = " `.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

pub fn cells_from_file(file_path: String) -> Vec<Cell> {
    let mut cells = Vec::default();

    let mut cell_char: Option<char> = None;
    let mut _space_char: Option<char> = None;

    let contents = fs::read_to_string(file_path).unwrap_or(String::default());
    
    // Figure out which character fills most space.
    for character in contents.chars() {
        if cell_char != None && _space_char != None {
            if ORDER.find(cell_char.unwrap()) < ORDER.find(_space_char.unwrap()) {
                (cell_char, _space_char) = (_space_char, cell_char);
            }
            break;
        }
        else if cell_char != None && character != cell_char.unwrap() {
            _space_char = Some(character);
        }
        else if cell_char == None {
            cell_char = Some(character);
        }
    }

    // Return empty cells collection if all characters are the same
    if cell_char == None || _space_char == None {
        return cells;
    }

    let lines = contents.lines();
    for (row, line) in lines.enumerate() {
        for (col, character) in line.chars().enumerate() {
            if character == cell_char.unwrap() {
                cells.push((col as i32, row as i32));
            } 
        }
    }
    cells
}
