extern crate rand;

use std::vec;
use rand::Rng;

pub enum CpuDifficulty {
	Easy,
	Medium,
	Advanced,
	Expert
}

pub struct CpuPlayer {
	pub difficulty: CpuDifficulty,
	pub marker: char,
}

impl CpuPlayer {
	pub fn new(difficulty: CpuDifficulty, marker: char) -> CpuPlayer {
		CpuPlayer {
			difficulty: difficulty,
			marker: marker,
		}
	}
	pub fn get_move(&self, board: &[char]) -> usize {
		let selection = match self.difficulty {
			CpuDifficulty::Easy => self.easy_move(board),
			CpuDifficulty::Medium => self.medium_move(board),
			CpuDifficulty::Advanced => self.easy_move(board),
			CpuDifficulty::Expert => self.easy_move(board),
		};
		selection
	}

	fn easy_move(&self, board: &[char]) -> usize {
		let mut available: Vec<u32> = Vec::new();
		for i in 0..board.len() {
			if board[i] == 'X' || board[i] == 'O' {
				continue;
			}

			available.push((i+1) as u32);
		}

		let mut random: usize;
		if available.len() == 1 {
			random = 0;
		} else {
			random = rand::thread_rng().gen_range(0, available.len()-1);
		}

		println!("The computer picked {}", available[random]);
		(available[random] as usize)
	}

	fn medium_move(&self, board: &[char]) -> usize {
		let check_block = rand::thread_rng().gen_range(0, 100);
		let mut selection;

		if check_block > 10 {
			let mut other_marker;
			if self.marker == 'X' {
				other_marker = 'O';
			} else {
				other_marker = 'X';
			}
			let (should_block, position) = check_for_winning_next_move(board, other_marker);
			if should_block {
				selection = position;
			} else {
				selection = self.easy_move(board);
			}
		} else {
			selection = self.easy_move(board);
		}
		
		selection
	}
}

fn check_for_winning_next_move(board: &[char], marker: char) -> (bool, usize) {
	let mut position: usize = 10;
	let mut done: bool = false;

	let other_marker = match marker {
		'X' => 'O',
		'O' => 'X',
		_ => panic!("WTF???"),
	};

	while !done {
		//012
		//345
		//678
		if board[0] == marker {
			if board[1] == marker {
				//xx2 
				if board[2] != other_marker {
					position = 2;
				}
			} 
			if board[2] == marker {
				//x1x
				if board[1] != other_marker {
					position = 1;
				}
			} 
			if board[3] == marker {
				//x
				//x
				//6
				if board[6] != other_marker {
					position = 6;
				}
			} 
			if board[6] == marker {
				//x
				//3
				//x
				if board[3] != other_marker {
					position = 3;
				}
			} 
			if board[4] == marker {
				//x
				//-x
				//--8
				if board[8] != other_marker {
					position = 8;
				}
			} 
			if board[8] == marker {
				//x
				//-4
				//--x
				if board[4] != other_marker {
					position = 4;
				}
			}
		}

		//012
		//345
		//678
		if board[1] == marker {
			if board[4] == marker {
				//-x
				//-x
				//-7
				if board[7] != other_marker {
					position = 7;
				}
			} 
			if board[7] == marker {
				//-x
				//-4
				//-x
				if board[4] != other_marker {
					position = 4;
				}
			} 
			if board[2] == marker {
				//0xx
				if board[0] != other_marker {
					position = 0;
				}
			}
		}

		//01x
		//345
		//678
		if board[2] == marker {
			if board[5] == marker {
				//--x
				//--x
				//--8
				if board[8] != other_marker {
					position = 8;
				}
			} 
			if board[8] == marker {
				//--x
				//--5
				//--x
				if board[5] != other_marker {
					position = 5;
				}
			} 
			if board[4] == marker {
				//--x
				//-x-
				//6--
				if board[6] != other_marker {
					position = 6;
				}
			}
			if board[6] == marker {
				//--x
				//-4-
				//x--
				if board[4] != other_marker {
					position = 4;
				}
			}
		}

		//012
		//x45
		//678
		if board[3] == marker {
			if board[4] == marker {
				//---
				//xx5
				//---
				if board[5] != other_marker {
					position = 5;
				}
			}
			if board[5] == marker {
				//---
				//x4x
				//---
				if board[4] != other_marker {
					position = 4;
				}
			}
			if board[6] == marker {
				//0--
				//x--
				//x--
				if board[0] != other_marker {
					position = 0;
				}
			}
		}

		//012
		//34x
		//678
		if board[5] == marker && board[8] == marker {
			//-2-
			//-x-
			//-x-
			if board[2] != other_marker {
				position = 2;
			}
		}

		if board[6] == marker && board[7] == marker {
			//---
			//---
			//xx8
			if board[8] != other_marker {
				position = 8;
			}
		}

		if board[6] == marker && board[8] == marker {
			//---
			//---
			//x7x
			if board[7] != other_marker {
				position = 7;
			}
		}

		if board[7] == marker && board[8] == marker {
			//---
			//---
			//6xx
			if board[6] != other_marker {
				position = 6;
			}
		}

		if position != 10 {
			done = true;
		} else {
			done = true;
		}
	}

	let block_winning_move = match position {
		10 => false,
		_ => true,
	};

	println!("Winning position {}", position);
	(block_winning_move, (position + 1) as usize)
}
