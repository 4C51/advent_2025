use std::array;

pub struct Dial<const N: usize> {
    pub range: [usize; N],
    pub current: i32,
    pub clicks: i32,
    pub zeros: i32
}

impl<const N: usize> Dial<N> {
    pub fn new() -> Dial<N> {
        Dial {
            range: array::from_fn(|i| i),
            current: 50,
            clicks: 0,
            zeros: 0
        }
    }

    pub fn turn(&mut self, cmd: &str) {
        let (dir, dist) = cmd.split_at(1);
        let dist: i32 = dist.parse().expect("Could not parse distance from command {cmd}");

        match dir {
            "R" => self.right(dist),
            "L" => self.left(dist),
            _ => panic!("Invalid command {cmd}"),
        }

        if self.current == 0 {
            self.zeros += 1;
        }
    }

    pub fn _reset(&mut self) {
        self.clicks = 0;
        self.zeros = 0;
    }

    fn left(&mut self, dist: i32) {
        for _ in 0..dist {
            self.current -= 1;
            
            match self.current {
                0 => self.clicks += 1,
                -1 => self.current = self.range.len() as i32 - 1,
                _ => continue,
            }
        }
    }

    fn right(&mut self, dist: i32) {
        for _ in 0..dist {
            self.current += 1;
            
            let max_val = self.range.len() as i32 - 1;
            match self.current {
                c if c > max_val => {
                    self.current = 0;
                    self.clicks += 1;
                },
                _ => continue,
            }
        }
    }
}