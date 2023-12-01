use std::fs;
use color_eyre::Report;
use std::str::FromStr;

fn main() -> Result<(), Report> {
    color_eyre::install()?;
    let input = fs::read_to_string("day_01/input.txt").unwrap();
    println!("{}", solution(&input));

    Ok(())
}

fn solution(input: &str) -> usize {
    input.lines()
        .filter(|e|!e.is_empty())
        .map(|line|{
            let mut it = line.chars().filter(char::is_ascii_digit);
            let first = it.next().unwrap().to_string();
            let last = it.last().map(|i|i.to_string()).unwrap_or(first.clone());
            first + &last
        })
        .map(|e|usize::from_str(&e).unwrap())
        .sum()
}


#[cfg(test)]
mod test {

    #[test]
    fn simple() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;
        assert_eq!(crate::solution(input), 142);
    }
}