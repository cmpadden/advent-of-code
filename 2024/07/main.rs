/*
--- Day 7: Bridge Repair ---

The Historians take you to a familiar rope bridge over a river in the middle of a jungle. The Chief
isn't on this side of the bridge, though; maybe he's on the other side?

When you go to cross the bridge, you notice a group of engineers trying to repair it. (Apparently,
it breaks pretty frequently.) You won't be able to cross until it's fixed.

You ask how long it'll take; the engineers tell you that it only needs final calibrations, but some
young elephants were playing nearby and stole all the operators from their calibration equations!
They could finish the calibrations if only someone could determine which test values could possibly
be produced by placing any combination of operators into their calibration equations (your puzzle
input).

For example:

190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20

Each line represents a single equation. The test value appears before the colon on each line; it is
your job to determine whether the remaining numbers can be combined with operators to produce the
test value.

Operators are always evaluated left-to-right, not according to precedence rules. Furthermore,
numbers in the equations cannot be rearranged. Glancing into the jungle, you can see elephants
holding two different types of operators: add (+) and multiply (*).

Only three of the above equations can be made true by inserting operators:

    190: 10 19 has only one position that accepts an operator: between 10 and 19. Choosing + would give 29, but choosing * would give the test value (10 * 19 = 190).
    3267: 81 40 27 has two positions for operators. Of the four possible configurations of the operators, two cause the right side to match the test value: 81 + 40 * 27 and 81 * 40 + 27 both equal 3267 (when evaluated left-to-right)!
    292: 11 6 16 20 can be solved in exactly one way: 11 + 6 * 16 + 20.

The engineers just need the total calibration result, which is the sum of the test values from just
the equations that could possibly be true. In the above example, the sum of the test values for the
three equations listed above is 3749.

Determine which equations could possibly be true. What is their total calibration result?
*/

use std::fs;
use std::io::{self};

pub fn p1(input: &str) -> i64 {
    fn perms(nums: &[i64], index: usize, current: i64, results: &mut Vec<i64>) {
        if index == nums.len() {
            results.push(current);
            return;
        }

        let next_num = nums[index];

        //println!("{:?}", (nums, index, current, &results));
        perms(nums, index + 1, current + next_num, results);
        perms(nums, index + 1, current * next_num, results);
    }

    let mut ans = 0;
    for line in input.lines() {
        println!("{:?}", line);
        let (left, right) = line.split_once(':').unwrap();

        let value = left.parse::<i64>().unwrap();

        let nums: Vec<i64> = right
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut results = Vec::new();
        perms(&nums, 1, nums[0], &mut results);

        if results.contains(&value) {
            ans += value;
        }
    }

    ans
}

/*
--- Part Two ---

The engineers seem concerned; the total calibration result you gave them is nowhere close to being
within safety tolerances. Just then, you spot your mistake: some well-hidden elephants are holding
a third type of operator.

The concatenation operator (||) combines the digits from its left and right inputs into a single
number. For example, 12 || 345 would become 12345. All operators are still evaluated left-to-right.

Now, apart from the three equations that could be made true using only addition and multiplication,
the above example has three more equations that can be made true by inserting operators:

    156: 15 6 can be made true through a single concatenation: 15 || 6 = 156.
    7290: 6 8 6 15 can be made true using 6 * 8 || 6 * 15.
    192: 17 8 14 can be made true using 17 || 8 + 14.

Adding up all six test values (the three that could be made before using only + and * plus the new
three that can now be made by also using ||) produces the new total calibration result of 11387.

Using your new knowledge of elephant hiding spots, determine which equations could possibly be
true. What is their total calibration result?
*/

pub fn p2(input: &str) -> i64 {
    fn perms(nums: &[i64], index: usize, current: i64, results: &mut Vec<i64>) {
        if index == nums.len() {
            results.push(current);
            return;
        }

        let next_num = nums[index];

        perms(nums, index + 1, current + next_num, results);
        perms(nums, index + 1, current * next_num, results);
        perms(
            nums,
            index + 1,
            format!("{}{}", current, next_num).parse::<i64>().unwrap(),
            results,
        );
    }

    let mut ans = 0;
    for line in input.lines() {
        println!("{:?}", line);
        let (left, right) = line.split_once(':').unwrap();

        let value = left.parse::<i64>().unwrap();

        let nums: Vec<i64> = right
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        let mut results = Vec::new();
        perms(&nums, 1, nums[0], &mut results);

        if results.contains(&value) {
            ans += value;
        }
    }

    ans
}

pub static INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1p2() {
        assert_eq!(p1(&INPUT), 3749);
        assert_eq!(p2(&INPUT), 11387);
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("p1 {}", p1(&input));
    println!("p2 {}", p2(&input));
    Ok(())
}
