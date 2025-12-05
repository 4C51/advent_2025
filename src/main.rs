use std::io;
use crate::safe::dial::Dial;
use crate::puzzle::input::{InputType, PuzzleInput};

mod safe;
mod puzzle;

fn main() {
    println!("Advent of Code 2025!");

    let mut mode: InputType = InputType::Puzzle;

    loop {
        let mut day = String::new();

        println!("Enter a day (1-?) {}- 'exit' or enter to quit - 'e' to toggle Example Mode", match mode { InputType::Example => "[EXAMPLE MODE] ", _ => ""});
        io::stdin()
            .read_line(&mut day)
            .expect("Failed to read input");

        let day = day.trim();
        let parsed_day: i32 = match day.parse() {
            Ok(num) => num,
            Err(_) => {
                if day.is_empty() || day == "exit" || day == "q" {
                    println!("Goodbye!");
                    break;
                } else if day == "e" {
                    println!("Toggling example mode!");
                    mode = match mode { InputType::Example => InputType::Puzzle, InputType::Puzzle => InputType::Example};
                    continue;
                } else {
                    println!("Please enter a number.");
                    continue;
                }
            },
        };

        println!("Loading day {parsed_day}...");

        match parsed_day {
            1 => day1(mode),
            2 => day2(mode),
            _ => println!("Nothing available for day {parsed_day}"),
        }
    }
}

fn day1(input_type: InputType) {
    println!("Determining password...");

    let mut dial: Dial<100> = Dial::new();
    let input = PuzzleInput::new(1);

    let lines = input.lines(input_type);

    lines.for_each(|line| {
        dial.turn(line);
    });

    println!("Final position: {}", dial.current);
    println!("Zeros: {}", dial.zeros);
    println!("Clicks: {}", dial.clicks);
}

fn day2(input_type: InputType) {
    println!("Determining invalid IDs...");
    let mut invalid_ids: Vec<String> = Vec::new();
    let mut invalid_total: i64 = 0;

    let input = PuzzleInput::new(2);
    let ranges: Vec<_> = input.as_string(input_type).split(",").collect();

    fn valid_id(id: &i64) -> bool {
        let id = id.to_string();
        let (first, second) = id.split_at(id.len()/2);
        return first != second;
    }

    for range in &ranges {
        let Some((start, end)) = range.split_once("-")
            else { panic!("Invalid range") };
        let start: i64 = start.parse().expect(format!("Could not parse start: {start}").as_str());
        let end: i64 = end.parse().expect(format!("Could not parse end: {end}").as_str());
        
        for id in start..=end {
            if !valid_id(&id) {
                invalid_ids.push(id.to_string());
                invalid_total += id;
            }
        }
    }

    println!("Invalid ids: {}", invalid_ids.join(", "));
    println!("Invalid total: {invalid_total}");

    println!("Part 2");

    fn valid_id_2(id: &i64) -> bool {
        let id = id.to_string();
        
        if id.len() == 1 { return true; }

        let double_id = id.clone() + &id;
        return !double_id[1..double_id.len() - 1].contains(&id);
    }

    invalid_ids.clear();
    invalid_total = 0;
    for range in &ranges {
        let Some((start, end)) = range.split_once("-")
            else { panic!("Invalid range") };
        let start: i64 = start.parse().expect(format!("Could not parse start: {start}").as_str());
        let end: i64 = end.parse().expect(format!("Could not parse end: {end}").as_str());
        
        for id in start..=end {
            if !valid_id_2(&id) {
                invalid_ids.push(id.to_string());
                invalid_total += id;
            }
        }
    }

    println!("Invalid ids: {}", invalid_ids.join(", "));
    println!("Invalid total: {invalid_total}");
}