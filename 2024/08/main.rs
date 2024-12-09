/*
--- Day 8: Resonant Collinearity ---

You find yourselves on the roof of a top-secret Easter Bunny installation.

While The Historians do their thing, you take a look at the familiar huge antenna. Much to your
surprise, it seems to have been reconfigured to emit a signal that makes people 0.1% more likely to
buy Easter Bunny brand Imitation Mediocre Chocolate as a Christmas gift! Unthinkable!

Scanning across the city, you find that there are actually many such antennas. Each antenna is
tuned to a specific frequency indicated by a single lowercase letter, uppercase letter, or digit.
You create a map (your puzzle input) of these antennas. For example:

............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............

The signal only applies its nefarious effect at specific antinodes based on the resonant
frequencies of the antennas. In particular, an antinode occurs at any point that is perfectly in
line with two antennas of the same frequency - but only when one of the antennas is twice as far
away as the other. This means that for any pair of antennas with the same frequency, there are two
antinodes, one on either side of them.

So, for these two antennas with frequency a, they create the two antinodes marked with #:

..........
...#......
..........
....a.....
..........
.....a....
..........
......#...
..........
..........

Adding a third antenna with the same frequency creates several more antinodes. It would ideally add
four antinodes, but two are off the right side of the map, so instead it adds only two:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......#...
..........
..........

Antennas with different frequencies don't create antinodes; A and a count as different frequencies.
However, antinodes can occur at locations that contain antennas. In this diagram, the lone antenna
with frequency capital A creates no antinodes but has a lowercase-a-frequency antinode at its
location:

..........
...#......
#.........
....a.....
........a.
.....a....
..#.......
......A...
..........
..........

The first example has antennas with two different frequencies, so the antinodes they create look
like this, plus an antinode overlapping the topmost A-frequency antenna:

......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#.

Because the topmost A-frequency antenna overlaps with a 0-frequency antinode, there are 14 total
unique locations that contain an antinode within the bounds of the map.

Calculate the impact of the signal. How many unique locations within the bounds of the map contain
an antinode?

*/

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self};

// For each antenna, find the _mirror_ location of all other matching antennas, and determine if
// the coordinates of that mirror are in the bounds of the map. Store the coordinate of that
// "anti-node".

pub fn p1(input: &str) -> usize {
    let lines: Vec<_> = input.lines().collect();

    let h = lines.len() as isize;
    let w = lines[0].len() as isize;

    let mut nodes = HashMap::new();
    for j in 0..lines.len() {
        for i in 0..lines[j].len() {
            let c = lines[j].chars().nth(i).unwrap();
            if c == '.' {
                continue;
            }
            nodes.entry(c).or_insert(Vec::new()).push((j, i));
        }
    }

    fn in_bounds(j: isize, i: isize, h: isize, w: isize) -> bool {
        i >= 0 && i < w && j >= 0 && j < h
    }

    let mut antinodes: HashSet<(isize, isize)> = HashSet::new();
    for (freq, coords) in &nodes {
        for ix1 in 0..coords.len() {
            for ix2 in 1..coords.len() {
                let (a1, b1) = (coords[ix1].0 as isize, coords[ix1].1 as isize);
                let (a2, b2) = (coords[ix2].0 as isize, coords[ix2].1 as isize);

                let (j1, i1) = (a1 - (a2 - a1), b1 - (b2 - b1));
                if in_bounds(j1, i1, h, w)
                    && lines[j1 as usize].chars().nth(i1 as usize) != Some(*freq)
                {
                    antinodes.insert((j1, i1));
                }

                let (j2, i2) = (a2 + (a2 - a1), b2 + (b2 - b1));
                if in_bounds(j2, i2, h, w)
                    && lines[j2 as usize].chars().nth(i2 as usize) != Some(*freq)
                {
                    antinodes.insert((j2, i2));
                }
            }
        }
    }

    println!("{:?}", antinodes);
    antinodes.len()
}

pub static INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1p2() {
        assert_eq!(p1(&INPUT), 14);
        //assert_eq!(p2(&INPUT), 11387);
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("p1 {}", p1(&input));
    //println!("p2 {}", p2(&input));
    Ok(())
}