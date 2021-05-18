use std::fs;

struct State {
    moves: Vec<char>,
    goal: bool,
}

fn main() {
    let board: Vec<Vec<char>> = fs::read_to_string("input/input_reduced.txt")
        .expect("Failed to read file!")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // let solution = solve(None, &board, &Vec::new(), 35, 4, 3);
    let solution = solve(None, &board, &Vec::new(), 18, 1, 1);

    println!("solution: {:?}", solution.moves);
}

fn not_repeating(moves: &Vec<char>, prev_cell: char, curr_move: char) -> bool {
    if let Some(&last) = moves.last() {
        if curr_move == 'L' && last == 'R' && prev_cell == 'F' {
            return false;
        } else if curr_move == 'R' && last == 'L' && prev_cell == 'F' {
            return false;
        } else if curr_move == 'U' && last == 'D' && prev_cell == 'F' {
            return false;
        } else if curr_move == 'D' && last == 'U' && prev_cell == 'F' {
            return false;
        }
    }

    true
}

fn state_changed(prev_board_opt: Option<&Vec<Vec<char>>>, curr_board: &Vec<Vec<char>>) -> bool {
    if let Some(prev_board) = prev_board_opt {
        return prev_board
            .iter()
            .zip(curr_board.iter())
            .any(|(a, b)| a.iter().zip(b.iter()).any(|(&x, &y)| x != y));
    } else {
        return true;
    }
}

fn solve(
    prev_board_opt: Option<&Vec<Vec<char>>>,
    board: &Vec<Vec<char>>,
    moves: &Vec<char>,
    steps: usize,
    i: usize,
    j: usize,
) -> State {
    // for debugging
    // if moves.len() >= 12
    //     && moves
    //         .iter()
    //         .zip(vec!['U', 'D', 'R', 'R', 'R', 'R', 'R', 'R', 'R', 'D', 'D', 'R'].iter())
    //         .all(|(&a, &b)| a == b)
    // {
    //     // for breakpoint purposes since CodeLLDB on VSCode doesn't handle conditional breakpoints properly (atleast for me, maybe I'm just doing it wrong)
    //     println!("moves are {:?}", moves);
    // }

    let state_has_changed = state_changed(prev_board_opt, board);

    if goals(board) {
        println!("successful: {:?}", moves);

        return State {
            moves: moves.clone(),
            goal: true,
        };
    } else if steps == 0 || board[i][j] == 'L' || !state_has_changed {
        // out of steps or steps into a laser or the same as prev_board
        // println!("failed: {:?}", moves);

        return State {
            moves: moves.clone(),
            goal: false,
        };
    }

    let mut left_result = State {
        moves: Vec::new(),
        goal: false,
    };
    let mut up_result = State {
        moves: Vec::new(),
        goal: false,
    };
    let mut right_result = State {
        moves: Vec::new(),
        goal: false,
    };
    let mut down_result = State {
        moves: Vec::new(),
        goal: false,
    };

    // left
    if j > 0 && (not_repeating(moves, board[i][j - 1], 'L') || state_has_changed) {
        let mut new_board = board.clone();
        let mut new_moves = moves.clone();
        new_moves.push('L');

        // nothing happens if wall or already reached goal
        if board[i][j - 1] == 'W' || board[i][j - 1] == '.' {
            left_result = State {
                moves: new_moves,
                goal: false,
            };
        } else if board[i][j - 1] == 'F' {
            // swap the position of Helltaker and the floor
            new_board[i][j - 1] = 'H';
            new_board[i][j] = 'F';
            left_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j - 1);
        } else if board[i][j - 1] == 'G' {
            // if goal, mark it as reached
            new_board[i][j - 1] = '.';
            left_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        } else if board[i][j - 1] == 'L' {
            // if laser, just proceed then the base case will handle the next step
            left_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j - 1);
        } else if board[i][j - 1] == 'B' {
            // if block
            if j > 1 {
                // if the left side of the block is floor, move the block
                if board[i][j - 2] == 'F' {
                    new_board[i][j - 2] = 'B';
                    new_board[i][j - 1] = 'F';
                } else if board[i][j - 2] == 'L' {
                    // if the left side of the block is a laser, "clean" the laser from that point downward

                    // mark the laser as a block then the previous block as the floor
                    new_board[i][j - 2] = 'B';
                    new_board[i][j - 1] = 'F';
                    for x in i + 1..board.len() {
                        if board[x][j - 2] == 'L' {
                            // mark it as floor
                            new_board[x][j - 2] = 'F';
                        }
                    }
                }

                // if the top of the block is laser, propagate the laser from that point downward
                if i > 0 && (board[i - 1][j - 1] == 'S' || board[i - 1][j - 1] == 'L') {
                    let mut x = i;

                    while x < board.len() && new_board[x][j - 1] == 'F' {
                        // mark it as laser
                        new_board[x][j - 1] = 'L';
                        x += 1;
                    }
                }
            }

            left_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        }
    }

    // up
    if i > 0 && (not_repeating(moves, board[i - 1][j], 'U') || state_has_changed) {
        let mut new_board = board.clone();
        let mut new_moves = moves.clone();
        new_moves.push('U');

        // nothing happens if wall or already reached goal
        if board[i - 1][j] == 'W' || board[i - 1][j] == '.' {
            up_result = State {
                moves: new_moves,
                goal: false,
            };
        } else if board[i - 1][j] == 'F' {
            // swap the position of Helltaker and the floor
            new_board[i - 1][j] = 'H';
            new_board[i][j] = 'F';
            up_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i - 1, j);
        } else if board[i - 1][j] == 'G' {
            // if goal, mark it as reached
            new_board[i - 1][j] = '.';
            up_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        } else if board[i - 1][j] == 'L' {
            // if laser, just proceed then the base case will handle the next step
            up_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i - 1, j);
        } else if board[i - 1][j] == 'B' {
            // if block
            if i > 1 {
                // if the top of the block is floor, move the block
                if board[i - 2][j] == 'F' {
                    new_board[i - 2][j] = 'B';
                    new_board[i - 1][j] = 'F';
                } else if board[i - 2][j] == 'L' {
                    // if the top of the block is a laser

                    // mark the laser as a block then the previously block as the floor
                    new_board[i - 2][j] = 'B';
                    new_board[i - 1][j] = 'F';
                }
            }
            up_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        }
    }

    // right
    if j < board[0].len() - 1 && (not_repeating(moves, board[i][j + 1], 'R') || state_has_changed) {
        let mut new_board = board.clone();
        let mut new_moves = moves.clone();
        new_moves.push('R');

        // nothing happens if wall or already reached goal
        if board[i][j + 1] == 'W' || board[i][j + 1] == '.' {
            up_result = State {
                moves: new_moves,
                goal: false,
            };
        } else if board[i][j + 1] == 'F' {
            // swap the position of Helltaker and the floor
            new_board[i][j + 1] = 'H';
            new_board[i][j] = 'F';
            right_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j + 1);
        } else if board[i][j + 1] == 'G' {
            // if goal, mark it as reached
            new_board[i][j + 1] = '.';
            right_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        } else if board[i][j + 1] == 'L' {
            // if laser, just proceed then the base case will handle the next step
            right_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j + 1);
        } else if board[i][j + 1] == 'B' {
            // if block
            if j < board[0].len() - 2 {
                // if the right side of the block is floor, move the block
                if board[i][j + 2] == 'F' {
                    new_board[i][j + 2] = 'B';
                    new_board[i][j + 1] = 'F';
                } else if board[i][j + 2] == 'L' {
                    // if the right side of the block is a laser, "clean" the laser from that point downward

                    // mark the laser as a block then the previous block as the floor
                    new_board[i][j + 2] = 'B';
                    new_board[i][j + 1] = 'F';
                    for x in i + 1..board.len() {
                        if board[x][j + 2] == 'L' {
                            // mark it as floor
                            new_board[x][j + 2] = 'F';
                        }
                    }
                }

                // if the top of the block is laser, propagate the laser from that point downward
                if i > 0 && (board[i - 1][j + 1] == 'S' || board[i - 1][j + 1] == 'L') {
                    let mut x = i;

                    while x < board.len() && new_board[x][j + 1] == 'F' {
                        // mark it as laser
                        new_board[x][j + 1] = 'L';
                        x += 1;
                    }
                }
            }
            right_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        }
    }

    // down
    if i < board.len() - 1 && (not_repeating(moves, board[i + 1][j], 'D') || state_has_changed) {
        let mut new_board = board.clone();
        let mut new_moves = moves.clone();
        new_moves.push('D');

        // nothing happens if wall or already reached goal
        if board[i + 1][j] == 'W' || board[i + 1][j] == '.' {
            down_result = State {
                moves: new_moves,
                goal: false,
            };
        } else if board[i + 1][j] == 'F' {
            // swap the position of Helltaker and the floor
            new_board[i + 1][j] = 'H';
            new_board[i][j] = 'F';
            down_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i + 1, j);
        } else if board[i + 1][j] == 'G' {
            // if goal, mark it as reached
            new_board[i + 1][j] = '.';
            down_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        } else if board[i + 1][j] == 'L' {
            // if laser, just proceed then the base case will handle the next step
            down_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i + 1, j);
        } else if board[i + 1][j] == 'B' {
            // if block
            if i < board.len() - 2 && board[i + 2][j] == 'F' {
                // if the bottom of the block is floor, move the block
                new_board[i + 2][j] = 'B';
                new_board[i + 1][j] = 'F';
            }
            down_result = solve(Some(&board), &new_board, &new_moves, steps - 1, i, j);
        }
    }

    if left_result.goal {
        return left_result;
    } else if right_result.goal {
        return right_result;
    } else if up_result.goal {
        return up_result;
    } else if down_result.goal {
        return down_result;
    } else {
        return State {
            moves: Vec::new(),
            goal: false,
        };
    }
}

fn goals(board: &Vec<Vec<char>>) -> bool {
    return !board.iter().any(|row| row.iter().any(|&cell| cell == 'G'));
}

// W - wall
// F - floor
// G - goal
// L - laser
// S - source of the laser
// B - block
// H - helltaker
// . - already reached goal
