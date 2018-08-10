pub struct Sudoku {
    pub board: Vec<u8>
}

impl Sudoku {
    pub fn new(board_string: &str) -> Sudoku {
        assert_eq!(board_string.len(), 81, "Board is not correct size!");
        let mut board = Vec::with_capacity(81);

        for (idx, character) in board_string.chars().enumerate() {
            match character.to_digit(10) {
                Some(n) => {
                    board.push(n as u8);
                },
                None => {
                    if character == '_' || character == '.' {
                        board.push(0);
                    } else {
                        panic!("Invalid character at index: {}", idx);
                    }
                }
            }
        }

        Sudoku {
            board
        }
    }

    pub fn solve(&self) -> Vec<u8> {
        let mut board_copy: Vec<u8> = self.board.clone();

        // Find the cells that need to be solved
        let mut blank_positions: Vec<usize> = self.board.iter().enumerate()
            .filter(|&(idx, val)| {
                return *val == 0;
            }).map(|(idx, val)| {
                return  idx;
            }).collect();

        let mut pointer_idx = 0;

        while pointer_idx < blank_positions.len() {
            let current_blank_pos: &usize = &blank_positions[pointer_idx];
            let mut target_cell_val: u8 = *&board_copy[*current_blank_pos as usize];
            target_cell_val += 1;
            target_cell_val %= 10;

            *board_copy.get_mut(*current_blank_pos).unwrap() = target_cell_val;

            //Backtrack after going through all possible values(1-9)
            if target_cell_val == 0 {
                pointer_idx -= 1;
                continue;
            }

            //Do validity check
            //Check row and column
            let target_x = *current_blank_pos % 9;
            let target_y = (*current_blank_pos - (*current_blank_pos % 9)) / 9;


            let mut passed_row_col_check = true;
            for i in 0..9 {
                //Sweep across the row and column
                let current_row_cell = target_y * 9 + i;
                let current_col_cell = target_x + (9 * i);

                //Do row check
                if current_row_cell != *current_blank_pos {
                    if *&board_copy[current_row_cell as usize] == target_cell_val {
                        passed_row_col_check = false;
                        break;
                    }
                }

                //Do column check
                if current_col_cell != *current_blank_pos {
                    if *&board_copy[current_col_cell as usize] == target_cell_val {
                        passed_row_col_check = false;
                        break;
                    }
                }
            }
            //Passed column and row check ... => Check the block now
            if passed_row_col_check {
                let mut passed_block_check = true;
                let block_top_left_cell = *current_blank_pos - (target_x % 3) - 9 * (target_y % 3);
                for y_offset in 0..3 {
                    for x_offset in 0..3 {
                        //Sweep through the block
                        let current_block_cell = block_top_left_cell + x_offset + 9 * y_offset;
                        if current_block_cell != *current_blank_pos {
                            if *&board_copy[current_block_cell as usize] == target_cell_val {
                                passed_block_check = false;
                                break;
                            }
                        }
                    }
                }

                //Increment the counter if all the block check also passed
                if passed_block_check {
                    pointer_idx += 1;
                }

            }
        }

        board_copy
    }

}