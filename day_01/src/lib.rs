use std::fs;
use color_eyre::Report;

const NUMBERS: &[(&str, &str)] = &[
	("one", "1"),
	("two", "2"),
	("three", "3"),
	("four", "4"),
	("five", "5"),
	("six", "6"),
	("seven", "7"),
	("eight", "8"),
	("nine", "9"),
];

pub fn solution(input: &str) -> u32 {
	let mut sum = 0;

	let mut left = None;
	let mut right = None;
	for char in input.chars() {
		match char {
			'0'..='9' => {
				let digit = char.to_digit(10).unwrap();
				if left.is_none() { left = Some(digit) } else { right = Some(digit) }
			}
			'\n' => {
				sum += left.unwrap() * 10 + right.or(left).unwrap();
				left = None;
				right = None;
			}
			_ => {}
		}
	}
	sum
}

fn solution_complex(input: &str) -> usize {
	todo!()
}


#[cfg(test)]
mod test {
	#[test]
	fn simple() {
		let input = "1abc2
		pqr3stu8vwx
		a1b2c3d4e5f
		treb7uchet
		";
		assert_eq!(crate::solution(input), 142);
	}

	#[test]
	fn complex() {
		let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
		assert_eq!(crate::solution_complex(input), 281);
	}
}