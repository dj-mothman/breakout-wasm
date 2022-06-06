use std::collections::VecDeque;

#[derive(Debug)]
pub struct Position {
	x: usize,
	y: usize,
}

pub enum Direction {
	Left,
	Right,
}

const BLOCK_SIZE: usize = 2;
const PADDLE_EXTRA: usize = 0;
const WIDTH: usize = 20;
const HEIGHT: usize = 10;

#[derive(Debug)]
pub struct PongGame {
	ball: Position,
	player: VecDeque<Position>,
	blocks: Vec<[Position; BLOCK_SIZE]>,
}

impl PongGame {
	pub fn new() -> Self {
		fn create_blocks() -> Vec<[Position; BLOCK_SIZE]> {
			let mut blocks: Vec<[Position; BLOCK_SIZE]> = Vec::new();
	
			for i in 0..HEIGHT/2 {
				for j in 0..WIDTH/BLOCK_SIZE {
					let start = j*BLOCK_SIZE;
					let end = start+BLOCK_SIZE;
					let block: [Position; BLOCK_SIZE] = (start..end).map(|x| Position{x, y: i}).collect::<Vec<_>>().try_into().expect("msg");
	
					blocks.push(block);
				}
			}
	
			return blocks;
		}

		let start = (WIDTH/2)-PADDLE_EXTRA;
		let end = (WIDTH/2)+PADDLE_EXTRA;

		Self {
			ball: Position{x: WIDTH/2, y: HEIGHT-1},
			blocks: create_blocks(),
			player: (start..=end).map(|x| Position{x, y: HEIGHT}).collect(),
		}
	}

	pub fn move_paddle(&mut self, direction: Direction){
		let front = self.player.get(0).unwrap().x;
		let back = self.player.get(2*PADDLE_EXTRA).unwrap().x;

		match direction {
			Direction::Left => if front > 0 {
				self.player.pop_back();
				self.player.push_front(Position{x:front-1, y:HEIGHT});
			},
			Direction::Right => if back < WIDTH {
				self.player.pop_front();
				self.player.push_back(Position{x:back+1, y:HEIGHT});
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::PongGame;
	use crate::Direction;

	#[test]
	fn test(){
		let mut game = PongGame::new();
		println!("{:?}", game.player);

		game.move_paddle(Direction::Left);
		println!("{:?}", game.player);

		game.move_paddle(Direction::Right);
		println!("{:?}", game.player);
	}
}