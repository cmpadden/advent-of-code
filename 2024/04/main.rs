/*
--- Day 4: Ceres Search ---

"Looks like the Chief's not here. Next!" One of The Historians pulls out a device and pushes the
only button on it. After a brief flash, you recognize the interior of the Ceres monitoring station!

As the search for the Chief continues, a small Elf who lives on the station tugs on your shirt;
she'd like to know if you could help her with her word search (your puzzle input). She only has to
find one word: XMAS.

This word search allows words to be horizontal, vertical, diagonal, written backwards, or even
overlapping other words. It's a little unusual, though, as you don't merely need to find one
instance of XMAS - you need to find all of them. Here are a few ways XMAS might appear, where
irrelevant characters have been replaced with .:

..X...
.SAMX.
.A..A.
XMAS.S
.X....

The actual word search will be full of letters instead. For example:

MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX

In this word search, XMAS occurs a total of 18 times; here's the same word search again, but where
letters not involved in any XMAS have been replaced with .:

....XXMAS.
.SAMXMS...
...S..A...
..A.A.MS.X
XMASAMX.MM
X.....XA.A
S.S.S.S.SS
.A.A.A.A.A
..M.M.M.MM
.X.X.XMASX

Take a look at the little Elf's word search. How many times does XMAS appear?
*/

/*
--- Part Two ---

The Elf looks quizzically at you. Did you misunderstand the assignment?

Looking for the instructions, you flip over the word search to find that this isn't actually an
XMAS puzzle; it's an X-MAS puzzle in which you're supposed to find two MAS in the shape of an X.
One way to achieve that is like this:

M.S .A. M.S

Irrelevant characters have again been replaced with . in the above diagram. Within the X, each MAS
can be written forwards or backwards.

Here's the same example from before, but this time all of the X-MASes have been kept instead:

.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........

In this example, an X-MAS appears 9 times.
*/

/*

cargo run

*/

use std::fs;
use std::io::{self};

pub fn p1(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let mut counter = 0;

    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            // vertical
            if j + 3 < grid.len() {
                let entry = (grid[j][i], grid[j + 1][i], grid[j + 2][i], grid[j + 3][i]);
                if entry == ('X', 'M', 'A', 'S') || entry == ('S', 'A', 'M', 'X') {
                    counter += 1;
                }
            }

            // horizontal
            if i + 3 < grid[j].len() {
                let entry = (grid[j][i], grid[j][i + 1], grid[j][i + 2], grid[j][i + 3]);
                if entry == ('X', 'M', 'A', 'S') || entry == ('S', 'A', 'M', 'X') {
                    counter += 1;
                }
            }

            // diagonal \
            if j + 3 < grid.len() && i + 3 < grid[j].len() {
                let entry = (
                    grid[j][i],
                    grid[j + 1][i + 1],
                    grid[j + 2][i + 2],
                    grid[j + 3][i + 3],
                );
                if entry == ('X', 'M', 'A', 'S') || entry == ('S', 'A', 'M', 'X') {
                    counter += 1;
                }
            }

            // diagonal /
            if j + 3 < grid.len() && i as isize - 3 >= 0 {
                let entry = (
                    grid[j][i],
                    grid[j + 1][i - 1],
                    grid[j + 2][i - 2],
                    grid[j + 3][i - 3],
                );
                if entry == ('X', 'M', 'A', 'S') || entry == ('S', 'A', 'M', 'X') {
                    counter += 1;
                }
            }
        }
    }

    counter
}

pub fn p2(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let mut counter = 0;

    for j in 0..grid.len() {
        for i in 0..grid[j].len() {
            if j + 2 < grid.len() && i + 2 < grid[j].len() {
                let e1 = (grid[j][i], grid[j + 1][i + 1], grid[j + 2][i + 2]);
                let e2 = (grid[j][i + 2], grid[j + 1][i + 1], grid[j + 2][i]);

                if (e1 == ('M', 'A', 'S') || e1 == ('S', 'A', 'M'))
                    && (e2 == ('M', 'A', 'S') || e2 == ('S', 'A', 'M'))
                {
                    counter += 1;
                }
            }
        }
    }

    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    static INPUT: &str = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX";

    #[test]
    fn test_p1() {
        assert_eq!(p1(&INPUT), 18);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(&INPUT), 9);
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("p1 {}", p1(&input));
    println!("p2 {}", p2(&input));
    Ok(())
}
