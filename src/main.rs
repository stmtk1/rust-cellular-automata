use std::fmt::{ self, Display };

#[derive(Clone, Debug)]
struct LifeGame {
    cells: Vec<Vec<bool>>,
}

impl Display for LifeGame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for col in &self.cells {
            let col_str: String = col
                .into_iter()
                .map(|&val| if val { '*' } else { '#' })
                .collect();
            write!(f, "{}\n", col_str)?;
        }

        write!(f, "\n")
    }
}

impl LifeGame {
    fn new() -> LifeGame {
        let cells = vec![
            vec![false, false, false, false, false, false, false, false, false, false ],
            vec![false, false, false, false, false, false, false, false, false, false ],
            vec![false, false, false, false, false, false, false, false, false, false ],
            vec![false, false, false, false, false, false, false, false, false, false ],
            vec![false, false, false, false, false, false, false, false, false, false ],
            vec![false, false, true,  true,  false, false, false, false, false, false ],
            vec![false, false, true,  true,  false, false, false, false, false, false ],
            vec![false, false, false, false, false, false, false, false, false, false ],
            vec![false, false, false, false, false, false, false, false, false, false ],
            vec![false, false, false, false, false, false, false, false, false, false ],
        ];

        LifeGame {
            cells,
        }
    }

    fn next_state(&self) -> LifeGame {
        let gen_col = |i: usize| (0..self.cells[i].len())
            .into_iter()
            .map(|j| self.next_cell_state(i, j))
            .collect();

        let cells = (0..self.cells.len())
            .into_iter()
            .map(|i| gen_col(i))
            .collect();

        LifeGame {
            cells,
        }
    }

    fn nth_after(&self, generation_num: usize) -> LifeGame {
        if generation_num == 0 {
            self.clone()
        } else {
            self.next_state().nth_after(generation_num - 1)
        }
    }

    fn next_cell_state(&self, col: usize, row: usize) -> bool {
        let around = self.count_around(col, row);

        if around < 2 {
            false
        } else if around > 3 {
            false
        } else if around == 3 {
            true
        } else {
            self.cells[col][row]
        }
    }

    fn count_around(&self, col: usize, row: usize) -> u8 {
        let mut ret = 0;
        for i in -1..2 {
            let col_pos = col as i64 + i;
            if col_pos < 0 || col_pos >= self.cells.len() as i64 {
                continue;
            }
            for j in -1..2 {
                let row_pos = row as i64 + j;
                if row_pos < 0 || row_pos >= self.cells[0].len() as i64 {
                    continue;
                }

                if i == 0 && j == 0 {
                    continue;
                }
                ret += self.cells[col_pos as usize][row_pos as usize] as u8;
            }
        }
        ret
    }
}

fn main() {
    let game = LifeGame::new().nth_after(10);
    println!("{}", game);
}
