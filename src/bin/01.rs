advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, |x| value(x, false))
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, |x| value(x, true))
}

fn solve(input: &str, value_fn: fn(&str) -> usize) -> Option<usize> {
    Some(input.lines().into_iter().map(value_fn).sum())
}

fn value(s: &str, p2: bool) -> usize {
    let nums = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let digits = s
        .char_indices()
        .into_iter()
        .fold((None, None), |acc, (i, x)| {
            let num = x.to_digit(10).and_then(|x| Some(x as usize)).or_else(|| {
                nums.iter()
                    .position(|&y| s[i..].starts_with(y))
                    .and_then(|x| Some(x + 1))
                    .filter(|_| p2)
            });
            (acc.0.or(num), num.or(acc.1))
        });
    digits.0.unwrap() * 10 + digits.1.unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
