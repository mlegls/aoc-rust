advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            Some(i + 1)
                .filter(|_| {
                    parse_line(l)
                        .iter()
                        .all(|(r, g, b)| *r <= 12 && *g <= 13 && *b <= 14)
                })
                .or(Some(0))
        })
        .sum()
}

pub fn part_two(input: &str) -> Option<u32> {
    input.lines().into_iter().map(|x| {
        parse_line(x)
            .into_iter()
            .reduce(|(r_, g_, b_), (r, g, b)| (r_.max(r), g_.max(g), b_.max(b)))
            .map(|(r, g, b)| r * g * b)
    }).sum()
}

fn parse_line(line: &str) -> Vec<(u32, u32, u32)> {
    let games: Vec<_> = line.split(":").last().unwrap().split(";").collect();
    games
        .into_iter()
        .map(|x| {
            x.split(",").into_iter().fold((0, 0, 0), |acc, c| {
                let pair: Vec<_> = c.split(' ').collect();
                // ["", num, color]
                match (pair[1], pair[2]) {
                    (n, "red") => (n.parse().unwrap(), acc.1, acc.2),
                    (n, "green") => (acc.0, n.parse().unwrap(), acc.2),
                    (n, "blue") => (acc.0, acc.1, n.parse().unwrap()),
                    _ => acc,
                }
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
