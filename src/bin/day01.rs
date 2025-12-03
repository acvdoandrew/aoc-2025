use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day01.txt").expect("failed to read input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));

    fn part1(input: &str) -> i64 {
        let mut position = 50;
        let mut count = 0;

        for line in input.lines() {
            let instruction = line.chars().next().expect("Not a character");
            let change: i64 = line[1..].parse().expect("Not a number");

            if instruction == 'R' {
                position = (position + change).rem_euclid(100);
            } else {
                position = (position - change).rem_euclid(100);
            }

            if position == 0 {
                count += 1;
            }
        }
        count
    }

    fn part2(input: &str) -> i64 {
        0
    }
}
