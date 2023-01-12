use pyo3::prelude::*;
use crate::parser::Type::{INTEGER, FLOAT, STRING, FUNCTION};

#[pyclass]
#[derive(Clone)]
pub enum Type {
    INTEGER,
    FLOAT,
    STRING,
    FUNCTION,
    PROGRAM
}

#[pyclass]
#[derive(Clone)]
pub struct Property {
    #[pyo3(get)]
    start: usize,

    #[pyo3(get)]
    end: usize,

    #[pyo3(get)]
    line: usize,

    #[pyo3(get, set)]
    val_type: Type,

    #[pyo3(get, set)]
    raw: String,

    #[pyo3(get, set)]
    children: Vec<Property>
}

fn is_integer(vec: &mut Vec<u8>) -> bool {
    // O(n)
    return vec.iter().all(|n| n.is_ascii_digit())
}

fn is_float(vec: &mut Vec<u8>) -> bool {
    // O(n)
    return vec.iter().all(|n| n.is_ascii_digit() || n == &46) // "."
}

fn get_type(buffer: &mut Vec<u8>) -> Type {
    // best O(n), worst O(2n)
    if is_float(buffer) {
        if is_integer(buffer) {
            INTEGER
        } else {
            FLOAT
        }
    } else {
        STRING
    }
}

fn vecu8_to_str(vecu8: Vec<u8>) -> String {
    return String::from_utf8(vecu8).unwrap();
}

pub fn parse(code: &String, val_type: Type, start: usize, line: usize, depth: usize, buffer: &mut Vec<u8>) -> Property {
    let mut arr = Property {
        val_type,
        raw: String::new(),
        children: vec![],
        start,
        end: start,
        line
    };

    let mut string_mode = false;
    let mut string_mode_i = 0;

    let mut i = start;
    while i < code.len() {
        let character = code.as_bytes().get(i).unwrap();

        if depth == 0 {
            match character {
                91 => {
                    // TODO: Duplicated code
                    let func = parse(code, FUNCTION, i + 1, arr.line, depth + 1, buffer);
                    i = func.end;
                    arr.line = func.line;

                    arr.children.push(func);
                }
                _ => buffer.push(*character)
            }

            // This pushes the buffer if we're at the end
        } else if string_mode {
            match character {
                34 => {
                    arr.children.push(Property {
                        start: string_mode_i,
                        end: i - 1,
                        line: arr.line,
                        val_type: STRING,
                        raw: vecu8_to_str(buffer.clone()),
                        children: vec![]
                    });
                    buffer.clear();
                    string_mode = !string_mode;
                }
                _ => buffer.push(*character)
            }
        } else {
            match character {
                91 => { // "["
                    let func = parse(code, FUNCTION, i + 1, arr.line, depth + 1, buffer);
                    i = func.end;
                    arr.line = func.line;

                    arr.children.push(func);
                }
                93 => { // "]"
                    if !buffer.is_empty() {
                        arr.children.push(Property {
                            start: i - buffer.len() - 1,
                            end: i,
                            line: arr.line,
                            val_type: get_type(buffer),
                            raw: vecu8_to_str(buffer.clone()),
                            children: vec![],
                        });
                        buffer.clear();
                    }

                    arr.start = arr.start.saturating_sub(1);
                    arr.end = i;
                    return arr;
                },
                34 => { // "
                    string_mode_i = i;
                    string_mode = !string_mode;
                }
                32 => { // " "
                    if !buffer.is_empty() {
                        arr.children.push(Property {
                            start: i - buffer.len(),
                            end: i - 1,
                            line: arr.line,
                            val_type: get_type(buffer),
                            raw: vecu8_to_str(buffer.clone()),
                            children: vec![],
                        });
                        buffer.clear();
                    }
                }
                // 10 => arr.line += 1, // "\n"
                _ => buffer.push(*character)
            }
        }
        i += 1;
    }

    // TODO: end-line character is only a problem in debug mode
    // if !buffer.is_empty() && !buffer.ends_with(&[10]) {
    if !buffer.is_empty() {
        arr.children.push(Property {
            start: i - buffer.len(),
            end: i - 1,
            line: arr.line,
            val_type: get_type(buffer),
            raw: vecu8_to_str(buffer.clone()),
            children: vec![],
        });
        buffer.clear();
    }
    return arr;
}
