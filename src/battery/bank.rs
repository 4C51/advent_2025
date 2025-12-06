pub struct Bank {
    batteries: Vec<u32>
}

impl Bank {
    pub fn new(input: &str) -> Bank {
        let batteries: Vec<u32> = input.chars().flat_map(|c| c.to_digit(10)).collect();
        Bank {
            batteries
        }
    }

    pub fn get_joltage(self, battery_count: usize) -> u64 {
        // let mut battery_weights: Vec<(&u32, usize)> = Vec::new();

        // for (i, battery) in self.batteries.iter().enumerate() {
        //     battery_weights.push((&battery, i));
        // }

        // let mut battery_weights: Vec<(&u32, usize)> = self.batteries.iter().enumerate().map(|(i, b)| (b, i)).collect();
        // print!("Unsorted: ");
        // battery_weights.iter().for_each(|b| print!("{0}", b.0));
        // println!();
        // battery_weights.sort_unstable_by_key(|bat| bat.0);
        // battery_weights.reverse();
        // print!("Value sorted: ");
        // battery_weights.iter().for_each(|b| print!("{0}", b.0));
        // println!();
        // battery_weights.sort_unstable_by_key(|bat| bat.1);
        // print!("Position sorted: ");
        // battery_weights.iter().for_each(|b| print!("{0}", b.0));
        // println!();
        let buffer = battery_count - 1;

        // let mut max_value: (&u32, usize) = (&1, 1);
        // for (i, bat) in self.batteries.iter().enumerate() {
        //     if bat > max_value.0 && (i < self.batteries.len() - buffer) {
        //         max_value.0 = bat;
        //         max_value.1 = i;
        //     }
        // }

        let mut start_index: usize = 0;
        let mut batteries: Vec<&u32> = Vec::new();

        for _ in 0..battery_count {
            let mut max_value: &u32 = &0;
            let remaining: usize = battery_count - batteries.len();

            let slice: String = self.batteries[start_index..].iter().map(|b| b.to_string()).collect();
                println!("Slice: {slice}; Remaining: {remaining}; Start index: {start_index}");
            for (i, bat) in self.batteries[start_index..].iter().enumerate() {
                if bat > max_value && (i < (slice.len() - remaining)) {
                    max_value = bat;
                    start_index = start_index+i+1;
                }
            }
            batteries.push(max_value);
        }

        let return_val: String = batteries.iter().map(|b| b.to_string()).collect();
        return return_val.parse().unwrap();

        // let mut second_max: &u32 = &1;
        // for bat in self.batteries.split_at(max_value.1 + 1).1.iter() {
        //     if bat > second_max {
        //         second_max = bat;
        //     }
        // }

        //return (max_value.0 * 10) + second_max;

    }
}