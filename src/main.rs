use std::{io::stdin};

fn main() {
    let game_over = false;
    let o = String::from("▢");
    let o_sign = &String::from("◯");
    let x_sign = &String::from("✖");
    let mut player = true;
    let mut plays = 0;

    let mut board: [[&String; 3]; 3] = [[&o, &o, &o], [&o, &o, &o], [&o, &o, &o]];

    println!("> TIC TAC TOE <");
    print_board(&board);

    println!("Playing as {}, choose your cell:", if player { &x_sign } else { &o_sign });

    'outer: while !game_over {
        let mut user_input = String::new();
        stdin().read_line(&mut user_input).expect("Failed to read user input");
        let trimmed = user_input.trim();
        match trimmed.parse::<u32>() {
            Ok(i) => {
                if i > 9 {
                    println!("Maximum number is 9");
                    continue
                }

                if plays == 8 {
                    println!("It's a tie!");
                    break 'outer;
                }

                let mut coord = 1;
                let board_c = board.clone();

                for (index, _item) in board_c.iter().enumerate() {
                    for (_index, _item) in board_c[index].iter().enumerate() {
                        if coord == i{
                            if !board[index][_index].eq_ignore_ascii_case(&o) {
                                println!("Spot is already played");
                                continue 'outer
                            }

                            board[index][_index] = if player { &x_sign } else { &o_sign };
                        }

                        coord += 1;
                    }
                }

                plays += 1;
                print_board(&board);

                for row in 0..3 {
                    let mut rows = 0;

                    for col in 0..3 {
                        if board[row][col].eq_ignore_ascii_case(&x_sign) {
                            rows += 1;
                        } else if board[row][col].eq_ignore_ascii_case(&o_sign) {
                            rows -= 1
                        }
                    }

                    if rows == 3 || rows == -3 {
                        println!("{} won the game!!!", if player { x_sign } else { o_sign });
                        break 'outer
                    }
                }

                for col in 0..3 {
                    let mut cols = 0;

                    for row in 0..3 {
                        if board[row][col].eq_ignore_ascii_case(&x_sign) {
                            cols += 1;
                        } else if board[row][col].eq_ignore_ascii_case(&o_sign) {
                            cols -= 1
                        }
                    }

                    if cols == 3 || cols == -3 {
                        println!("{} won the game!!!", if player { x_sign } else { o_sign });
                        break 'outer
                    }
                }

                let mut diag1 = 0;
                for col in 0..3 {
                    if board[col][col].eq_ignore_ascii_case(&x_sign) {
                        diag1 += 1;
                    } else if board[col][col].eq_ignore_ascii_case(&o_sign) {
                        diag1 -= 1
                    }

                    if diag1 == 3 || diag1 == -3 {
                        println!("{} won the game!!!", if player { x_sign } else { o_sign });
                        break 'outer
                    }
                }

                let mut diag2 = 0;
                for col in 0..3 {
                    if board[col][2-col].eq_ignore_ascii_case(&x_sign) {
                        diag2 += 1;
                    } else if board[col][2-col].eq_ignore_ascii_case(&o_sign) {
                        diag2 -= 1
                    }

                    if diag2 == 3 || diag2 == -3 {
                        println!("{} won the game!!!", if player { x_sign } else { o_sign });
                        break 'outer
                    }
                }

                player = !player;

                println!("Playing as {}, choose your cell:", if player { x_sign } else { o_sign });
            },
            Err(..) => println!("This was not an integer {}", trimmed),
        };
    }
}

fn print_board(board: &[[&String; 3]; 3]) {
    println!("_______________");
    for i in 0..board.len() {
        println!("{:?}", board[i]);
    }
    println!("¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯");
}