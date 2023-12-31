use connect_four::board::{Board, GameState, PlayerColor};

pub fn start_game(board_width: usize, board_height: usize) {
    let mut board = Board::new(board_width, board_height);

    let mut active_color = PlayerColor::Red;

    while board.get_state() == GameState::Ongoing {

        board.draw_to_console();
        
        println!("It is {:?}'s turn!", active_color);

        loop {
            let mut input = String::new();

            println!("Please enter the column you want to drop your coin (positive integer): ");

            std::io::stdin()
                .read_line(&mut input)
                .expect("Input should be valid!");

            let input: usize = match input.trim().parse::<usize>() {
                Ok(value) => match value.checked_sub(1) {
                    Some(value) => value,
                    None => {
                        println!("Please enter a positive integer!");
                        continue;
                    }
                },
                Err(_) => {
                    println!("Input a valid integer!");
                    continue;
                }
            };

            if !board.can_play_move(input) {
                println!("Cannot put a coin into file {input}!");
                continue;
            }

            board.play_move(input, active_color.clone());

            break;
        }

        active_color = if active_color == PlayerColor::Red {
            PlayerColor::Yellow
        } else {
            PlayerColor::Red
        };
    }

    board.draw_to_console();

    if board.get_state() == GameState::Draw {
        println!("The game ended in a draw!");
    } else if let GameState::Win(color) = board.get_state() {
        println!("{:?} color won!", color);
    }
}
