use std::fmt::Display;
use std::{thread, time::Duration};

pub struct Conway {
    state: Vec<Vec<bool>>,
}

impl Display for Conway {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut state_str = String::new();
        for i in &self.state {
            for j in i {
                if *j {
                    state_str.push_str(" ●");
                } else {
                    state_str.push_str(" ·");
                }
            }
            state_str.push_str("\n");
        }
        write!(f, "{}", state_str)
    }
}

impl Conway {
    pub fn new(state: Vec<Vec<bool>>) -> Self {
        Conway { state }
    }

    fn get_neighbours(&self, ii: i32, jj: i32) -> Vec<bool> {
        let validate = |i: i32, j: i32| -> bool {
            i >= 0 && i < self.state.len() as i32 && j >= 0 && j < self.state[0].len() as i32
        };
        let mut neighbours_state = vec![];
        #[rustfmt::skip]
        let neighbours_lookup = vec![
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),  /*  */   (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];
        for (x, y) in neighbours_lookup {
            let x = ii + x;
            let y = jj + y;
            if validate(x, y) {
                neighbours_state.push(self.state[x as usize][y as usize]);
            }
        }
        neighbours_state
    }

    fn dead_or_alive(&self, i: i32, j: i32) -> bool {
        let neighbours = self.get_neighbours(i, j);
        let mut alive_neighbours = 0;
        for n in neighbours {
            if n {
                alive_neighbours += 1;
            }
        }

        if self.state[i as usize][j as usize] {
            alive_neighbours == 2 || alive_neighbours == 3
        } else {
            alive_neighbours == 3
        }
    }

    fn update_state(&mut self) {
        let mut new_state_row1: Vec<bool> = vec![];
        let mut new_state_row2: Vec<bool> = vec![];
        let len = self.state.len();
        for i in 0..len {
            if i > 1 {
                self.state[i - 2] = new_state_row1;
            }
            new_state_row1 = new_state_row2;
            new_state_row2 = vec![];
            for j in 0..self.state[i].len() {
                new_state_row2.push(self.dead_or_alive(i as i32, j as i32));
            }
        }
        self.state[len - 2] = new_state_row1;
        self.state[len - 1] = new_state_row2;
    }

    pub fn print_loop(mut self) {
        let frames_per_sec = 8;
        println!("{}", self);
        print!("\x1B[2J\x1B[1;1H");
        loop {
            thread::sleep(Duration::from_millis(1000 / frames_per_sec));
            print!("\x1B[2J\x1B[1;1H");
            self.update_state();
            print!("{}", self);
        }
    }

    pub fn gosper_glider_gun() -> Self {
        let mut state = vec![];
        for _ in 0..25 {
            let mut row = vec![];
            for _ in 0..40 {
                row.push(false);
            }
            state.push(row);
        }

        #[rustfmt::skip]
        let initial_alive_cells = vec![
            (5, 1), (5, 2), (6, 1), (6, 2), (6, 10), (6, 11),
            (5, 11), (6, 11), (7, 11), (4, 12), (5, 12), (6, 12),
            (7, 12), (8, 12), (3, 13), (4, 13), (8, 13), (9, 13),
            (5, 17), (6, 17), (7, 17), (5, 18), (6, 18), (7, 18),
            (4, 20), (3, 21), (5, 21), (2, 22), (6, 22), (3, 23),
            (4, 23), (5, 23), (1, 24), (2, 24), (6, 24), (7, 24),
            (3, 35), (4, 35), (3, 36), (4, 36), (23, 37), (23, 38),
            (24, 37), (24, 38),
        ];

        for (i, j) in initial_alive_cells {
            state[i][j] = true;
        }

        Self::new(state)
    }
}

#[cfg(test)]
mod tests {
    use super::Conway;

    #[test]
    fn conway_state_display() {
        let c = Conway::new(vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ]);
        assert_eq!(" · ● ·\n · ● ·\n · ● ·\n", c.to_string());
    }

    #[test]
    fn conway_next_state() {
        let mut c1 = Conway::new(vec![
            vec![false, true, false],
            vec![false, true, false],
            vec![false, true, false],
        ]);
        c1.update_state();
        let updated_state = c1.to_string();
        assert_eq!(" · · ·\n ● ● ●\n · · ·\n", updated_state);

        let mut c2 = Conway::new(vec![
            vec![false, false, false, false],
            vec![false, true, true, false],
            vec![false, false, true, false],
            vec![false, false, false, false],
        ]);
        c2.update_state();
        let updated_state = c2.to_string();
        assert_eq!(" · · · ·\n · ● ● ·\n · ● ● ·\n · · · ·\n", updated_state);
    }
}
