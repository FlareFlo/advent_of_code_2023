use std::cmp::Ordering;
use std::str::FromStr;
use color_eyre::Report;

fn main() -> Result<(), Report> {
	color_eyre::install()?;

	Ok(())
}

#[derive(Debug)]
struct Games {
	games: Vec<Game>,
}

#[derive(Debug)]
struct Game {
	id: usize,
	contents: Vec<BagContent>,
}


#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
struct BagContent {
	red: usize,
	green: usize,
	blue: usize,
}


impl Ord for BagContent {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.red > other.red && self.green > other.green && self.blue > other.blue {
			Ordering::Greater
		} else if self.red < other.red && self.green < other.green && self.blue < other.blue {
			Ordering::Less
		} else {
			Ordering::Equal
		}
	}
}

impl BagContent {
	pub fn from_str(input: &str) -> Self {
		let mut red = 0;
		let mut green = 0;
		let mut blue = 0;

		let get_color = |color_count: &str| {
			usize::from_str(color_count.split_once(" ").unwrap().0).unwrap()
		};

		for color_count in input.split(",") {
			let color_count = color_count.trim();
			match () {
				_ if color_count.ends_with("red") => {
					red = get_color(color_count);
				}
				_ if color_count.ends_with("green") => {
					green = get_color(color_count);
				}
				_ if color_count.ends_with("blue") => {
					blue = get_color(color_count);
				}
				_ => { unreachable!() }
			}
		}

		Self {
			red,
			green,
			blue,
		}
	}
}

impl Game {
	fn from_str(s: &str) -> Self {
		let split = s.split_once(" ").unwrap().1.split_once(":").unwrap();
		let id = usize::from_str(split.0).unwrap();
		let contents = split.1.split(";")
			.map(BagContent::from_str)
			.collect();

		Self {
			id,
			contents,
		}
	}

	pub fn max_game(&self) -> BagContent {
		let red = self.contents.iter().max_by_key(|e| e.red).map(|e| *e).unwrap();
		let blue = self.contents.iter().max_by_key(|e| e.blue).map(|e| *e).unwrap();
		let green = self.contents.iter().max_by_key(|e| e.green).map(|e| *e).unwrap();
		BagContent {
			red: red.red,
			green: green.green,
			blue: blue.blue,
		}
	}
}

impl Games {
	pub fn from_lines(input: &str) -> Self {
		let contents = input.lines()
			.map(|line| Game::from_str(line))
			.collect();

		Self {
			games: contents,
		}
	}

	pub fn playable_move(&self, max: BagContent) -> usize {
		self.games.iter()
			.map(|e| (e.id, e.max_game()))
			.filter(|e| e.1 <= max)
			// .map(|e|{println!("{}", e.0); e})
			.map(|e| e.0)
			.sum()
	}
}

fn solve_simple(input: &str) -> usize {
	let game = Games::from_lines(input);
	let max = BagContent {
		red: 12,
		green: 13,
		blue: 14,
	};
	game.playable_move(max)
}

#[cfg(test)]
mod test {
	use std::fs;
	use crate::{solve_simple};

	#[test]
	fn test_simple() {
		let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
		assert_eq!(solve_simple(input), 8);
	}


	#[test]
	fn test_full() {
		let input = fs::read_to_string("input.txt").unwrap();
		let res = solve_simple(&input);
		assert_ne!(res, 3853);
		panic!("{res}")
	}
}