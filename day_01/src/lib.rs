// From left to right:
// bytes for literal integer
// literal length
// byte for ascii repr
// bit-identifier
const NUMBERS: [([u8; 5], u8, u8, u16); 9] = [
	(*b"one  ", 3, b'1', ONE),
	(*b"two  ", 3, b'2', TWO),
	(*b"three", 5, b'3', THREE),
	(*b"four ", 4, b'4', FOUR),
	(*b"five ", 4, b'5', FIVE),
	(*b"six  ", 3, b'6', SIX),
	(*b"seven", 5, b'7', SEVEN),
	(*b"eight", 5, b'8', EIGHT),
	(*b"nine ", 4, b'9', NINE),
];

const ALL: u16 = ONE | TWO | THREE | FOUR | FIVE | SIX | SEVEN | EIGHT | NINE;
const ONE: u16 = 1 << 0;
const TWO: u16 = 1 << 1;
const THREE: u16 = 1 << 2;
const FOUR: u16 = 1 << 3;
const FIVE: u16 = 1 << 4;
const SIX: u16 = 1 << 5;
const SEVEN: u16 = 1 << 6;
const EIGHT: u16 = 1 << 7;
const NINE: u16 = 1 << 8;

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

fn replace_number_to_digit(input: &mut [u8]) -> usize {
	let len = input.len();
	let mut candidates = ALL;
	for i in 0..5_usize {
		// dbg!(String::from_utf8_lossy(input));
		if i >= input.len() { return 1; }
		for (bytes, byte_len, num, pat) in NUMBERS {
			if bytes[i] != input[i] {
				candidates &= !pat;
			} else {
				if candidates.count_ones() == 1 &&  byte_len as usize <= len && bytes[..byte_len as usize] == input[..byte_len as usize] {

					// Remove when not debugging
					for i in 1..byte_len {
						input[i as usize] = b'_';
					}


					input[0] = num;
					return byte_len as usize;
				}
			}
		}
	}
	1
}

fn replace_numbers_to_digits(input: &mut [u8]) {
	let mut lines = vec![];
	let mut last = 0;
	for (i, char) in input.iter().enumerate() {
		if *char == b'\n' {
			lines.push(last..i);
			last  = i + 1;
		}
	}
	// dbg!(lines.iter().map(|e|String::from_utf8_lossy(&input[e.clone()])).collect::<Vec<_>>());
	for line in lines {
		// println!("{}", String::from_utf8_lossy(&input[line.clone()]));
		if line.len() < 3 {
			continue;
		}
		let line_cropped = &mut input[line.clone()];
		let mut at = 0;
		while at < line_cropped.len() {
			at += replace_number_to_digit(&mut line_cropped[at..]);
		}
		// println!("{}", String::from_utf8_lossy(&input[line]));
	}
}

pub fn solution_complex(input: &mut [u8]) -> u32 {
	replace_numbers_to_digits(input);
	solution(input)
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
".to_owned().into_bytes();
		assert_eq!(crate::solution_complex(&mut input), 281);
	}

	#[test]
	fn run_simple() {
		let file = fs::read("input.txt").unwrap();
		println!("{}", crate::solution(&file));
	}

	#[test]
	fn run_complex() {
		let mut file = fs::read("input.txt").unwrap();
		let res = crate::solution_complex(&mut file);
		assert_ne!(res, 54538); // Confirmed wrong
		println!("{}", res);
	}
}