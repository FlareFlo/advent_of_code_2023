use std::mem::size_of;

const ONE: (u64, u64) = (u64::from_ne_bytes(*b"one\0\0\0\0\0"), u64::from_ne_bytes([255, 255, 255, 0, 0, 0, 0, 0]));
const TWO: (u64, u64) = (u64::from_ne_bytes(*b"two\0\0\0\0\0"), u64::from_ne_bytes([255, 255, 255, 0, 0, 0, 0, 0]));
const THREE: (u64, u64) = (u64::from_ne_bytes(*b"three\0\0\0"), u64::from_ne_bytes([255, 255, 255, 255, 255, 0, 0, 0]));
const FOUR: (u64, u64) = (u64::from_ne_bytes(*b"four\0\0\0\0"), u64::from_ne_bytes([255, 255, 255, 255, 0, 0, 0, 0]));
const FIVE: (u64, u64) = (u64::from_ne_bytes(*b"five\0\0\0\0"), u64::from_ne_bytes([255, 255, 255, 255, 0, 0, 0, 0]));
const SIX: (u64, u64) = (u64::from_ne_bytes(*b"six\0\0\0\0\0"), u64::from_ne_bytes([255, 255, 255, 0, 0, 0, 0, 0]));
const SEVEN: (u64, u64) = (u64::from_ne_bytes(*b"seven\0\0\0"), u64::from_ne_bytes([255, 255, 255, 255, 255, 0, 0, 0]));
const EIGHT: (u64, u64) = (u64::from_ne_bytes(*b"eight\0\0\0"), u64::from_ne_bytes([255, 255, 255, 255, 255, 0, 0, 0]));
const NINE: (u64, u64) = (u64::from_ne_bytes(*b"nine\0\0\0\0"), u64::from_ne_bytes([255, 255, 255, 255, 0, 0, 0, 0]));

const NUMS: [(u64, u64); 9] = [ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

pub fn solution(input: &[u8]) -> u32 {
	let mut sum = 0;

	let mut left = None;
	let mut right = None;
	for char in input {
		match char {
			b'1'..=b'9' => {
				if left.is_none() { left = Some(*char as u32 - 48) } else { right = Some(*char as u32 - 48) }
			}
			b'\n' => {
				sum += left.unwrap() * 10 + right.or(left).unwrap();
				left = None;
				right = None;
			}
			_ => {}
		}
	}
	sum
}


pub fn solution_complex(input: &str) -> u32 {
	input.lines().map(|l| solve_line(l.as_bytes()) as u32).sum()
}

fn solve_line(input: &[u8]) -> u8 {
	let mut left = None;
	let mut right = None;
	let mut left_offset = input.len();
	let mut right_offset = 0;

	for (i, byte) in input.into_iter().enumerate() {
		if byte.is_ascii_digit() {
			if left.is_none() {
				left = Some(*byte - 48);
				left_offset = i;
			}
			right = Some(*byte - 48);
			right_offset = i;
		}
	}

	let mut i = 0;

	while i < input.len() {
		let index = {
			i..(i + size_of::<usize>()).min(input.len())
		};

		let bytes = &input[index.clone()];
		let mut to_be_bits = [0, 0, 0, 0, 0, 0, 0, 0];
		to_be_bits[..index.len()].copy_from_slice(bytes);


		let bits = u64::from_ne_bytes(to_be_bits.try_into().unwrap());

		let next_match = find_match(bits);
		if let Some(matched) = next_match {
			if i < left_offset {
				left = Some(matched);
				left_offset = i;
			}
			if i > right_offset {
				right = Some(matched);
				right_offset = i;
			}
			// i += (8 - bits.count_zeros())
			i += 1;
		} else {
			i += 1;
		}
	}
	left.unwrap() * 10 + right.or(left).unwrap()
}

fn find_match(bits: u64) -> Option<u8> {
	for (x, (number, mask)) in NUMS.into_iter().enumerate() {
		if number == (mask & bits) {
			// println!("Found {}!", String::from_utf8_lossy(&number.to_ne_bytes()));
			return Some(x as u8 + 1);
		}
	}
	None
}

#[cfg(test)]
mod test {
	use std::fs;

	#[test]
	fn simple() {
		let input = "1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet
		";
		assert_eq!(crate::solution(input.as_bytes()), 142);
	}

	#[test]
	fn complex() {
		let mut input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
		assert_eq!(crate::solution_complex(&mut input), 281);
	}

	#[test]
	fn test_line_two1nine() {
		assert_eq!(29, crate::solve_line(b"two1nine"));
	}

	#[test]
	fn test_line_eightwothree() {
		assert_eq!(83, crate::solve_line(b"eightwothree"));
	}

	#[test]
	fn test_line_abcone2threexyz() {
		assert_eq!(13, crate::solve_line(b"abcone2threexyz"));
	}

	#[test]
	fn test_line_xtwone3four() {
		assert_eq!(24, crate::solve_line(b"xtwone3four"));
	}

	#[test]
	fn test_line_4nineeightseven2() {
		assert_eq!(42, crate::solve_line(b"4nineeightseven2"));
	}

	#[test]
	fn test_line_zoneight234() {
		assert_eq!(14, crate::solve_line(b"zoneight234"));
	}

	#[test]
	fn test_line_7pqrstsixteen() {
		assert_eq!(76, crate::solve_line(b"7pqrstsixteen"));
	}

	#[test]
	fn run_simple() {
		let file = fs::read("input.txt").unwrap();
		assert_eq!(crate::solution(&file), 56049);
	}

	#[test]
	fn run_complex() {
		let mut file = fs::read_to_string("input.txt").unwrap();
		let res = crate::solution_complex(&mut file);
		assert_eq!(res, 54530);
	}
}