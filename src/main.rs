extern crate rand;

use std::io;
use std::char;
use rand::Rng;

mod ai;

use ai::CpuDifficulty;

enum PlayerType {
	Human,
	Computer
}

enum Player {
	PlayerOne,
	PlayerTwo
}

struct PlayerInfo {
	player: Player,
	player_type: PlayerType
}

struct GameState {
	current_player: Player,
	cpu_player: ai::CpuPlayer,
	board: [char; 9],
	player_one_type: PlayerType,
	player_two_type: PlayerType,
	in_progress: bool
}

fn main() {
	let mut state = GameState {
		current_player: Player::PlayerOne,
		cpu_player: ai::CpuPlayer::new(CpuDifficulty::Medium, 'O'),
		board: ['0', '0', '0', '0', '0', '0', '0', '0', '0'],
		player_one_type: PlayerType::Human,
		player_two_type: PlayerType::Computer,
		in_progress: false
	};

	let mut input;

	println!("Welcome to Rust Tic-tac-toe!");

	while !state.in_progress {
		println!("How many players? (1 or 2) ");

		input = String::new();
		io::stdin().read_line(&mut input)
			.ok()
			.expect("Failed to read line");

		let players: u32 = match input.trim().parse() {
			Ok(num) => num,
			Err(_) => 15
		};

		match players {
			1 => println!("OK! One Human and one Computer player!"),
			2 => println!("OK! Two Human players!"),
			_ => println!("Not a valid option.  Please enter 1 or 2!"),
		}

		if players == 1 {
			determine_first_player(&mut state);
		}

		state.in_progress = match players {
			1 | 2 => true,
			_ => false
		};
	}

	init_board(&mut state);
	run_game(&mut state);

	println!("Thanks for playing!");
}

fn init_board(state: &mut GameState) {
	for i in 0..state.board.len() {
		let digit = (i as u32) + 1;
		let board_val = char::from_digit(digit, 10);
		state.board[i] = board_val.unwrap();
	}
}

fn print_board(board: &[char]) {
	println!("/-----------------\\");
	println!("|     |     |     |");
	println!("|  {}  |  {}  |  {}  |", board[0], board[1], board[2]);
	println!("|     |     |     |");
	println!("|-----------------|");
	println!("|     |     |     |");
	println!("|  {}  |  {}  |  {}  |", board[3], board[4], board[5]);
	println!("|     |     |     |");
	println!("|-----------------|");
	println!("|     |     |     |");
	println!("|  {}  |  {}  |  {}  |", board[6], board[7], board[8]);
	println!("|     |     |     |");
	println!("\\-----------------/");
}

fn determine_first_player(state: &mut GameState) {
	let cpu_player = rand::thread_rng().gen_range(0, 100);

	if cpu_player < 50 {
		state.player_one_type = PlayerType::Computer;
		state.player_two_type = PlayerType::Human;
		state.cpu_player = ai::CpuPlayer::new(CpuDifficulty::Medium, 'X');
	} else {
		state.player_one_type = PlayerType::Human;
		state.player_two_type = PlayerType::Computer;
	}
}

fn run_game(state: &mut GameState) {

	state.current_player = Player::PlayerOne;

	loop {
		print_board(&state.board);
		let mut selection: usize;
		match state.current_player {
			Player::PlayerOne => {
				selection = match state.player_one_type {
					PlayerType::Human => get_selection_for_player(&state.current_player),
					PlayerType::Computer => state.cpu_player.get_move(&state.board),
				};
			},
			Player::PlayerTwo => {
				selection = match state.player_two_type {
					PlayerType::Human => get_selection_for_player(&state.current_player),
					PlayerType::Computer => state.cpu_player.get_move(&state.board),
				}
			}
		}

		match selection {
			1 ... 9 => {},
			_ => { println!("Please pick a valid spot!"); continue; }
		}

		let mut valid_selection: bool;
		let selection: usize = selection - 1;
		match state.board[selection] as char {
			'X' | 'O' => { println!("Spot taken, please choose another spot"); continue; },
			_ => valid_selection = true
		}

		if valid_selection {
			state.board[selection] = match state.current_player {
				Player::PlayerOne => 'X',
				Player::PlayerTwo => 'O'
			};

			state.current_player = match state.current_player {
				Player::PlayerOne => Player::PlayerTwo,
				Player::PlayerTwo => Player::PlayerOne
			};
		}

		let (winner, finished) = update_game_status(&state.board);
		if finished {
			print_board(&state.board);
			if winner == 1 {
				println!("Player 1 wins!");
			} else if winner == 2 {
				println!("Player 2 wins!");
			} else {
				println!("The game was a draw!");
			}

			break;
		}
	}
}

fn get_selection_for_player(player: &Player) -> usize {
	let selection: usize;
	let mut input = String::new();

	match *player {
		Player::PlayerOne => {
			println!("Player 1, pick an open spot:");
			io::stdin().read_line(&mut input)
				.ok()
				.expect("Failed to read line");

			selection = match input.trim().parse() {
				Ok(num) => num,
				Err(_) => 15
			}
		},
		Player::PlayerTwo => {
			println!("Player 2, pick and open spot:");
			io::stdin().read_line(&mut input)
				.ok()
				.expect("Failed to read line");

			selection = match input.trim().parse() {
				Ok(num) => num,
				Err(_) => 15
			}
		}
	}

	selection
}

fn get_selection_for_computer(board: &[char]) -> usize
{
	let mut available = [0; 9];
	let mut index = 0;
	for i in 0..board.len() {
		if board[i] == 'X' || board[i] == 'O' {
			continue;
		}
		available[index] = i + 1;
		//print!("{}", i);
		index += 1;	
	}

	//println!("");

	let random: usize = rand::thread_rng().gen_range(0, index-1);
	println!("The computer picked {}", available[random]);
	available[random]
}

fn update_game_status(board: &[char]) -> (u32, bool) {
	let mut finished = false;
	let mut winner = 0;
	let player_marker = ['X', 'O'];

	//first check to see if X has won
	for marker in &player_marker {
		winner += 1;
		if board[0] == *marker {
			if board[1] == *marker && board[2] == *marker { 
				finished = true;
				break;
			} else if board[3] == *marker && board[6] == *marker {
				finished = true;
				break;
			} else if board[4] == *marker && board[8] == *marker {
				finished = true;
				break;
			}
		}

		if board[1] == *marker {
			if board[4] == *marker && board[7] == *marker {
				finished = true;
				break;
			}
		}

		if board[2] == *marker {
			if board[5] == *marker && board[8] == *marker {
				finished = true;
				break;
			} else if board[4] == * marker && board[6] == *marker {
				finished = true;
				break;
			}
		}

		if board[3] == *marker {
			if board[4] == *marker && board[5] == *marker {
				finished = true;
				break;
			} 
		}

		if board[6] == *marker && board[7] == *marker && board[8] == *marker {
			finished = true;
			break;
		}
	}

	if !finished {
		finished = true;
		winner = 3;
		for val in board {
			match *val {
				'X' | 'O' => continue,
				_ => { finished = false; break; }
			}
		}
	}

	(winner, finished)
}
