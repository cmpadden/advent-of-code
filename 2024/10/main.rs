/*
--- Day 10: Hoof It ---

You all arrive at a Lava Production Facility on a floating island in the sky. As the others begin
to search the massive industrial complex, you feel a small nose boop your leg and look down to
discover a reindeer wearing a hard hat.

The reindeer is holding a book titled "Lava Island Hiking Guide". However, when you open the book,
you discover that most of it seems to have been scorched by lava! As you're about to ask how you
can help, the reindeer brings you a blank topographic map of the surrounding area (your puzzle
input) and looks up at you excitedly.

Perhaps you can help fill in the missing hiking trails?

The topographic map indicates the height at each position using a scale from 0 (lowest) to 9
(highest). For example:

0123
1234
8765
9876

Based on un-scorched scraps of the book, you determine that a good hiking trail is as long as
possible and has an even, gradual, uphill slope. For all practical purposes, this means that a
hiking trail is any path that starts at height 0, ends at height 9, and always increases by a
height of exactly 1 at each step. Hiking trails never include diagonal steps - only up, down, left,
or right (from the perspective of the map).

You look up from the map and notice that the reindeer has helpfully begun to construct a small pile
of pencils, markers, rulers, compasses, stickers, and other equipment you might need to update the
map with hiking trails.

A trailhead is any position that starts one or more hiking trails - here, these positions will
always have height 0. Assembling more fragments of pages, you establish that a trailhead's score is
the number of 9-height positions reachable from that trailhead via a hiking trail. In the above
example, the single trailhead in the top left corner has a score of 1 because it can reach a single
9 (the one in the bottom left).

This trailhead has a score of 2:

...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9

(The positions marked . are impassable tiles to simplify these examples; they do not appear on your
actual topographic map.)

This trailhead has a score of 4 because every 9 is reachable via a hiking trail except the one
immediately to the left of the trailhead:

..90..9
...1.98
...2..7
6543456
765.987
876....
987....

This topographic map contains two trailheads; the trailhead at the top has a score of 1, while the
trailhead at the bottom has a score of 2:

10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01

Here's a larger example:

89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732

This larger example has 9 trailheads. Considering the trailheads in reading order, they have scores
of 5, 6, 5, 3, 1, 3, 5, 3, and 5. Adding these scores together, the sum of the scores of all
trailheads is 36.

The reindeer gleefully carries over a protractor and adds it to the pile. What is the sum of the
scores of all trailheads on your topographic map?
*/

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self};

pub fn p1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let h = grid.len();
    let w = grid[0].len();

    // Store full path `Vec<i32>` that end in `9` originating at `(i32, i32)`

    // coordiante and value of grid element
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();

    // construct `HashMap` of coordinates
    for j in 0..h {
        for i in 0..w {
            let n = grid[j][i];
            if grid[j][i] != '.' {
                map.insert((j as i32, i as i32), n.to_digit(10).unwrap() as i32);
            }
        }
    }

    fn traverse(
        map: &HashMap<(i32, i32), i32>,
        coord: (i32, i32),
        previous_value: i32,
        paths: &mut HashSet<(i32, i32)>,
    ) {
        let n = map.get(&coord);
        if n.is_none() {
            return;
        }

        let value = n.unwrap();
        if value - previous_value != 1 {
            return;
        }

        //println!("{:?}", (value, coord, &paths));

        if *value == 9 as i32 {
            paths.insert(coord);
            return;
        }

        // traverse the map up, down, left, and right
        traverse(map, (coord.0 + 1, coord.1), *n.unwrap(), paths);
        traverse(map, (coord.0 - 1, coord.1), *n.unwrap(), paths);
        traverse(map, (coord.0, coord.1 + 1), *n.unwrap(), paths);
        traverse(map, (coord.0, coord.1 - 1), *n.unwrap(), paths);
    }

    map.iter()
        .filter(|(_, value)| **value == 0)
        .map(|(key, _)| {
            let mut paths: HashSet<(i32, i32)> = HashSet::new();
            traverse(&map, *key, -1, &mut paths);
            paths.len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const T1: &str = "\
...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9";

    const T2: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_p1p2() {
        assert_eq!(p1(&T1), 2);
        assert_eq!(p1(&T2), 36);
        //assert_eq!(p2(&INPUT), 34);
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("p1 {}", p1(&input));
    //println!("p2 {}", p2(&input));
    Ok(())
}
