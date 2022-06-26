use std::{collections::{VecDeque, self}, iter};

#[derive(Debug)]
pub struct Position {
	x: i8,
	y: i8,
}

#[derive(Debug)]
pub struct Ball {
	position: Position,
	x_vel: i8,
	y_vel: i8,
}

#[derive(Debug)]
pub struct Block {
	positions: [Position; BLOCK_SIZE],
	destroyed: bool
}

pub enum Direction {
	Left,
	Right,
}

const BLOCK_SIZE: usize = 2;
const PADDLE_EXTRA: u8 = 2;
const WIDTH: u8 = 20;
const HEIGHT: u8 = 10;

#[derive(Debug)]
pub struct PongGame {
	ball: Ball,
	player: VecDeque<Position>,
	blocks: Vec<[Position; BLOCK_SIZE]>,
}

impl PongGame {
	pub fn new() -> Self {
		fn create_blocks() -> Vec<[Position; BLOCK_SIZE]> {
			let mut blocks: Vec<[Position; BLOCK_SIZE]> = Vec::new();
	
			for i in 0..HEIGHT/2 {
				for j in 0..WIDTH as usize/BLOCK_SIZE {
					let start = j*BLOCK_SIZE;
					let end = start+BLOCK_SIZE;
					let block: [Position; BLOCK_SIZE] = (start..end).map(|x| Position{x: x as i8, y: i as i8}).collect::<Vec<_>>().try_into().expect("msg");

					blocks.push(block);
				}
			}
	
			return blocks;
		}

		let start = (WIDTH/2)-PADDLE_EXTRA;
		let end = (WIDTH/2)+PADDLE_EXTRA;

		Self {
			ball: Ball{position: Position{x: (WIDTH/2) as i8, y: (HEIGHT-1) as i8}, x_vel: 0, y_vel: 0},
			blocks: create_blocks(),
			player: (start..=end).map(|x| Position{x: x as i8, y: HEIGHT as i8}).collect(),
		}
	}

	pub fn move_paddle(&mut self, direction: Direction) {
		let front = self.player.get(0).unwrap().x;
		let back = self.player.get(2*PADDLE_EXTRA as usize).unwrap().x;

		match direction {
			Direction::Left => if self.ball.x_vel == 0 && self.ball.y_vel == 0 {
				self.ball.x_vel = -1;
				self.ball.y_vel = -1;
			} else if front > 0 {
				self.player.pop_back();
				self.player.push_front(Position{x:front-1, y:HEIGHT as i8});
			},
			Direction::Right => if self.ball.x_vel == 0 && self.ball.y_vel == 0 {
				self.ball.x_vel = 1;
				self.ball.y_vel = -1;
			} else if back < WIDTH as i8 {
				self.player.pop_front();
				self.player.push_back(Position{x:back+1, y:HEIGHT as i8});
			},
		}
	}

	pub fn move_ball(&mut self) {
		let mut next_ball_pos = Position {
			x: self.ball.position.x + self.ball.x_vel,
			y: self.ball.position.y + self.ball.y_vel,
		};

		if next_ball_pos.x < 0 || next_ball_pos.x > WIDTH as i8 {
			self.ball.x_vel = -self.ball.x_vel;
			next_ball_pos.x = self.ball.position.x + self.ball.x_vel;
		}

		if next_ball_pos.y < 0 || self.player.iter().any(|p| next_ball_pos.x == p.x && next_ball_pos.y == p.y)  {
			self.ball.y_vel = -self.ball.y_vel;
			next_ball_pos.y = self.ball.position.y + self.ball.y_vel;
		}
		else if next_ball_pos.y > HEIGHT as i8 {
			println!("game over");
		}

		for i in 0..self.blocks.len() - 1 {
			if self.blocks[i].iter().any(|p| next_ball_pos.x == p.x && next_ball_pos.y == p.y) {

				if self.blocks[i][0].x > self.ball.position.x || self.blocks[i][self.blocks[i].len() - 1].x < self.ball.position.x {					
					self.ball.x_vel = -self.ball.x_vel;
					next_ball_pos.x = self.ball.position.x + self.ball.x_vel;
				}

				if self.blocks[i][0].y > self.ball.position.y || self.blocks[i][self.blocks[i].len() - 1].y < self.ball.position.y {					
					self.ball.y_vel = -self.ball.y_vel;
					next_ball_pos.y = self.ball.position.y + self.ball.y_vel;
				}

				println!("removed: {:?}", self.blocks[i]);
				self.blocks.remove(i);
			}
		}
		
		self.ball.position = next_ball_pos;		
	}
}

#[cfg(test)]
mod tests {
	use crate::PongGame;
	use crate::Direction;

	#[test]
	fn test(){
		let mut game = PongGame::new();

		game.move_paddle(Direction::Left);
		println!("{:?}", game.player);
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);

		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);
		game.move_ball();
		println!("{:?}", game.ball);
		println!("{:?}", game.blocks);


	}
}