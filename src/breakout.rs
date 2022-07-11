use std::{collections::{VecDeque}, fmt};

#[derive(Debug, Clone, Copy)]
pub struct Position {
	pub x: i8,
	pub y: i8,
}

#[derive(Debug, Clone, Copy)]
pub struct Ball {
	pub position: Position,
	x_vel: i8,
	y_vel: i8,
}

#[derive(Debug)]
pub struct Block {
	pub positions: [Position; BLOCK_SIZE],
	pub color: Color
}

pub enum Direction {
	Left,
	Right,
}

#[derive(Debug)]
pub enum Color {
	Red,
	Green,
	Blue,
	Yellow,
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let emoji = match self {
			Color::Red => '🟥',
			Color::Green => '🟩',
			Color::Blue => '🟦',
			Color::Yellow => '🟨',
		};

		write!(f, "{}", emoji)
		// or, alternatively:
		// fmt::Debug::fmt(self, f)
	}
}

pub const BLOCK_SIZE: usize = 2;
pub const PADDLE_EXTRA: u8 = 2;

#[derive(Debug)]
pub struct BreakoutGame {
	pub ball: Ball,
	pub player: VecDeque<Position>,
	pub blocks: Vec<Block>,
	pub width: u8,
	pub height: u8
}

impl BreakoutGame {
	pub fn new (width: u8, height: u8) -> Self {
		let get_color = |x| {
			match x {
				0 => Color::Red,
				1 => Color::Green,
				2 => Color::Blue,
				_ => Color::Yellow,
			}
		};

		let mut color_counter = 0;

		let mut create_blocks = || {
			let mut blocks: Vec<Block> = Vec::new();
	
			for i in 0..height/2 {
				for j in 0..width as usize/BLOCK_SIZE {
					let start = j*BLOCK_SIZE;
					let end = start+BLOCK_SIZE;
					let block_array: [Position; BLOCK_SIZE] = (start..end).map(|x| Position{x: x as i8, y: i as i8}).collect::<Vec<_>>().try_into().expect("msg");
					
					color_counter = if color_counter+1 > 3 { 0 } else { color_counter+1 };
					let color = get_color(color_counter);

					let block = Block { positions: block_array, color: color };
					blocks.push(block);
				}
			}
	
			return blocks;
		};

		let start = (width/2)-PADDLE_EXTRA;
		let end = (width/2)+PADDLE_EXTRA;

		Self {
			ball: Ball{position: Position{x: (width/2) as i8, y: (height-2) as i8}, x_vel: 0, y_vel: 0},
			blocks: create_blocks(),
			player: (start..=end).map(|x| Position{x: x as i8, y: (height-1) as i8}).collect(),
			width: width,
			height: height,
		}
	}

	pub fn move_paddle(&mut self, direction: Direction) {
		let front = self.player.get(0).unwrap().x;
		let back = self.player.get(2*PADDLE_EXTRA as usize).unwrap().x;

		match direction {
			Direction::Left => {
				if self.ball.x_vel == 0 && self.ball.y_vel == 0 {
					self.ball.x_vel = -1;
					self.ball.y_vel = -1;
				}
				if front > 0 {
					self.player.pop_back();
					self.player.push_front(Position{x:front-1, y:(self.height-1) as i8});
				}
			},
			Direction::Right => {
				if self.ball.x_vel == 0 && self.ball.y_vel == 0 {
					self.ball.x_vel = 1;
					self.ball.y_vel = -1;
				}
				if back < (self.width-1) as i8 {
					self.player.pop_front();
					self.player.push_back(Position{x:back+1, y:(self.height-1) as i8});
				}
			},
		}
	}

	fn get_positions_to_check(&self, next_ball_pos: Position) -> Vec<Position> {
		let mut positions_to_check = Vec::new();
		if self.ball.x_vel > 0 { 
			positions_to_check.push(Position {x: self.ball.position.x + 1, y: self.ball.position.y})
		} else if self.ball.x_vel < 0 {			
			positions_to_check.push(Position {x: self.ball.position.x - 1, y: self.ball.position.y})
		}
		if self.ball.y_vel > 0 { 
			positions_to_check.push(Position {x: self.ball.position.x, y: self.ball.position.y + 1})
		} else if self.ball.x_vel < 0 {			
			positions_to_check.push(Position {x: self.ball.position.x, y: self.ball.position.y - 1})
		}		
		positions_to_check.push(next_ball_pos);
		positions_to_check
	}

	fn check_positions(&mut self, positions_to_check: Vec<Position>, mut next_ball_pos: Position) {
		for p in 0..positions_to_check.len() {
			let check_pos = positions_to_check.get(p).unwrap();
			for i in 0..self.blocks.len() - 1 {
				if self.blocks[i].positions.iter().any(|p| check_pos.x == p.x && check_pos.y == p.y) {
	
					if self.blocks[i].positions[0].x > self.ball.position.x || self.blocks[i].positions[self.blocks[i].positions.len() - 1].x < self.ball.position.x {					
						self.ball.x_vel = -self.ball.x_vel;
						next_ball_pos.x = self.ball.position.x + self.ball.x_vel;
					}
	
					if self.blocks[i].positions[0].y != self.ball.position.y {					
						self.ball.y_vel = -self.ball.y_vel;
						next_ball_pos.y = self.ball.position.y + self.ball.y_vel;
					}
	
					println!("removed: {:?}", self.blocks[i]);
					self.blocks.remove(i);

					let new_pos_to_check = self.get_positions_to_check(next_ball_pos);
					self.check_positions(new_pos_to_check, next_ball_pos);

					return;
				}
			}
		}
	}

	pub fn move_ball(&mut self) {
		let mut next_ball_pos = Position {
			x: self.ball.position.x + self.ball.x_vel,
			y: self.ball.position.y + self.ball.y_vel,
		};

		if next_ball_pos.x < 0 || next_ball_pos.x >= self.width as i8 {
			self.ball.x_vel = -self.ball.x_vel;
			next_ball_pos.x = self.ball.position.x + self.ball.x_vel;
		}

		if next_ball_pos.y < 0 || self.player.iter().any(|p| next_ball_pos.x == p.x && next_ball_pos.y == p.y)  {
			self.ball.y_vel = -self.ball.y_vel;
			next_ball_pos.y = self.ball.position.y + self.ball.y_vel;
		}
		else if next_ball_pos.y >= self.height as i8 {
			println!("game over");
		}

		let positions_to_check = self.get_positions_to_check(next_ball_pos);
		self.check_positions(positions_to_check, next_ball_pos);		
		self.ball.position = next_ball_pos;		
	}

	pub fn pos_iter(&self) -> impl Iterator<Item = Position> + '_ {
		return (0..self.height).flat_map(move |y| (0..self.width).map(move |x| Position {x: x as i8, y: y as i8}))
	}

	pub fn tick(&mut self) {
		self.move_ball();
	}
}

#[cfg(test)]
mod tests {
	use crate::breakout::BreakoutGame;
	use crate::breakout::Direction;

	#[test]
	fn test(){
		let mut game = BreakoutGame::new(20, 10);

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