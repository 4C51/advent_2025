use std::{fs, io::{self, Error}, ptr::null};

#[derive(Copy, Clone)]
pub enum InputType {
    Example,
    Puzzle
}

pub struct PuzzleInput {
    example_input: String,
    puzzle_input: String,
}

const INPUT_DIR: &str = "./input/";

impl PuzzleInput {
    pub fn new(day: i32) -> Result<PuzzleInput, io::Error> {
        let example_file = match fs::read_to_string(format!("{INPUT_DIR}day_{day}_example.txt"))
            {
                Ok(file) => file.trim().to_string(),
                Err(_) => {
                    return Err(Error::new(io::ErrorKind::NotFound, "Could not find example file"));
                }
            };

        let puzzle_file = fs::read_to_string(format!("{INPUT_DIR}day_{day}.txt"))
            .expect("Could not read puzzle input file").trim().to_string();

        Ok(PuzzleInput {
            example_input: example_file,
            puzzle_input: puzzle_file
        })
    }

    pub fn lines(&self, input_type: InputType) -> std::str::Lines<'_> {
        match input_type {
            InputType::Example => self.example_input.lines(),
            InputType::Puzzle => self.puzzle_input.lines()
        }
    }

    pub fn as_string(&self, input_type: InputType) -> &String {
        match input_type {
            InputType::Example => &self.example_input,
            InputType::Puzzle => &self.puzzle_input
        }
    }
}