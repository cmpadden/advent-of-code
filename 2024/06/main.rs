/*
--- Day 6: Guard Gallivant ---

The Historians use their fancy device again, this time to whisk you all away to the North Pole
prototype suit manufacturing lab... in the year 1518! It turns out that having direct access to
history is very convenient for a group of historians.

You still have to be careful of time paradoxes, and so it will be important to avoid anyone from
1518 while The Historians search for the Chief. Unfortunately, a single guard is patrolling this
part of the lab.

Maybe you can work out where the guard will go ahead of time so that The Historians can search
safely?

You start by making a map (your puzzle input) of the situation. For example:

....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...

The map shows the current position of the guard with ^ (to indicate the guard is currently facing
up from the perspective of the map). Any obstructions - crates, desks, alchemical reactors, etc. -
are shown as #.

Lab guards in 1518 follow a very strict patrol protocol which involves repeatedly following these
steps:

    If there is something directly in front of you, turn right 90 degrees.
    Otherwise, take a step forward.

Following the above protocol, the guard moves up several times until she reaches an obstacle (in
this case, a pile of failed suit prototypes):

....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Because there is now an obstacle in front of the guard, she turns right before continuing straight
in her new facing direction:

....#.....
........>#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...

Reaching another obstacle (a spool of several very long polymers), she turns right again and
continues downward:

....#.....
.........#
..........
..#.......
.......#..
..........
.#......v.
........#.
#.........
......#...

This process continues for a while, but the guard eventually leaves the mapped area (after walking
    past a tank of universal solvent):

....#.....
.........#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#v..

By predicting the guard's route, you can determine which specific positions in the lab will be in the patrol path. Including the guard's starting position, the positions visited by the guard before leaving the area are marked with an X:

....#.....
....XXXXX#
....X...X.
..#.X...X.
..XXXXX#X.
..X.X.X.X.
.#XXXXXXX.
.XXXXXXX#.
#XXXXXXX..
......#X..

In this example, the guard will visit 41 distinct positions on your map.

Predict the path of the guard. How many distinct positions will the guard visit before leaving the
mapped area?
*/

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{self};

pub fn p1(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    for line in &lines {
        println!("{:?}", line);
    }

    let h = lines.len() as i32;
    let w = lines[0].len() as i32;

    // Derive current position of the guard
    let mut pos: (i32, i32) = (0, 0);
    for (j, line) in lines.iter().enumerate() {
        if let Some(i) = line.find('^') {
            pos = (j as i32, i as i32);
        }
    }

    fn in_bounds(pos: (i32, i32), h: i32, w: i32) -> bool {
        pos.0 >= 0 && pos.0 < h && pos.1 >= 0 && pos.1 < w
    }

    let mut dir: usize = 0;
    let mut unique_pos: HashSet<(i32, i32)> = HashSet::new();
    unique_pos.insert(pos); // include starting position

    loop {
        let prev = pos.clone();
        match dir % 4 {
            0 => pos.0 -= 1, // ^
            1 => pos.1 += 1, // >
            2 => pos.0 += 1, // v
            3 => pos.1 -= 1, // <
            _ => panic!("Invalid direction"),
        }

        println!("{:?}", pos);

        if !in_bounds(pos, h, w) {
            break;
        }

        println!(
            "{:?}",
            (pos, h, w, lines[pos.0 as usize].chars().nth(pos.1 as usize))
        );
        if lines[pos.0 as usize].chars().nth(pos.1 as usize) == Some('#') {
            dir += 1;
            pos = prev; // revert position
        }

        unique_pos.insert(pos);
    }

    unique_pos.len() as i32
}

/*
--- Part Two ---

While The Historians begin working around the guard's patrol route, you borrow their fancy device
and step outside the lab. From the safety of a supply closet, you time travel through the last few
months and record the nightly status of the lab's guard post on the walls of the closet.

Returning after what seems like only a few seconds to The Historians, they explain that the guard's
patrol area is simply too large for them to safely search the lab without getting caught.

Fortunately, they are pretty sure that adding a single new obstruction won't cause a time paradox.
They'd like to place the new obstruction in such a way that the guard will get stuck in a loop,
making the rest of the lab safe to search.

To have the lowest chance of creating a time paradox, The Historians would like to know all of the
possible positions for such an obstruction. The new obstruction can't be placed at the guard's
starting position - the guard is there right now and would notice.

In the above example, there are only 6 different positions where a new obstruction would cause the
guard to get stuck in a loop. The diagrams of these six situations use O to mark the new
obstruction, | to show a position where the guard moves up/down, - to show a position where the
guard moves left/right, and + to show a position where the guard moves both up/down and left/right.

Option one, put a printing press next to the guard's starting position:

....#.....
....+---+#
....|...|.
..#.|...|.
....|..#|.
....|...|.
.#.O^---+.
........#.
#.........
......#...

Option two, put a stack of failed suit prototypes in the bottom right quadrant of the mapped area:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
......O.#.
#.........
......#...

Option three, put a crate of chimney-squeeze prototype fabric next to the standing desk in the bottom right quadrant:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----+O#.
#+----+...
......#...

Option four, put an alchemical retroencabulator near the bottom left corner:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
..|...|.#.
#O+---+...
......#...

Option five, put the alchemical retroencabulator a bit to the right instead:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
....|.|.#.
#..O+-+...
......#...

Option six, put a tank of sovereign glue right next to the tank of universal solvent:

....#.....
....+---+#
....|...|.
..#.|...|.
..+-+-+#|.
..|.|.|.|.
.#+-^-+-+.
.+----++#.
#+----++..
......#O..

It doesn't really matter what you choose to use as an obstacle so long as you and The Historians
can put it into position without the guard noticing. The important thing is having enough options
that you can find one that minimizes time paradoxes, and in this example, there are 6 different
positions you could choose.

You need to get the guard stuck in a loop by adding a single new obstruction. How many different
positions could you choose for this obstruction?
*/

pub fn p2(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let h = lines.len() as i32;
    let w = lines[0].len() as i32;

    //fn find_starting_pos(lines: &Vec<String>) {
    //}

    // Derive current position of the guard
    let mut pos: (i32, i32) = (0, 0);
    for (j, line) in lines.iter().enumerate() {
        if let Some(i) = line.find('^') {
            pos = (j as i32, i as i32);
        }
    }

    fn in_bounds(pos: (i32, i32), h: i32, w: i32) -> bool {
        pos.0 >= 0 && pos.0 < h && pos.1 >= 0 && pos.1 < w
    }

    let mut infinite_counter = 0;
    let starting_pos = pos.clone();

    // We love a good brute force...
    for ix in 0..h * w {
        let mock_block_pos = (ix / w, ix % w);

        println!("{:?}", (mock_block_pos, h, w, infinite_counter));

        // Skip replacement of starting position
        if mock_block_pos == starting_pos {
            continue;
        }

        let mut dir: usize = 0;
        let mut visited: HashMap<(i32, i32), usize> = HashMap::new();
        pos = starting_pos;

        let mut fallback_counter = 0;
        loop {
            if fallback_counter > 1_000_000 {
                println!("Fallback infinite loop, what's going on...");
                infinite_counter += 1;
                break;
            }
            fallback_counter += 1;

            visited.insert(pos, dir % 4); // starting point
            let prev = pos.clone();
            match dir % 4 {
                0 => pos.0 -= 1, // ^
                1 => pos.1 += 1, // >
                2 => pos.0 += 1, // v
                3 => pos.1 -= 1, // <
                _ => panic!("Invalid direction"),
            }

            if !in_bounds(pos, h, w) {
                break;
            }

            let c = lines[pos.0 as usize].chars().nth(pos.1 as usize);
            if c == Some('#') || (pos == mock_block_pos) {
                dir += 1;
                pos = prev; // revert position
            }

            // check if we have already visited the square, and are moving in the same direction; this
            // indicates that we're in an infinite loop...
            if visited.get(&pos) == Some(&(dir % 4)) {
                infinite_counter += 1;
                break;
            }
        }
    }

    infinite_counter
}

pub static INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

pub static INPUT2: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
...#^.....
........#.
#.........
......#...";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1p2() {
        //assert_eq!(p1(&INPUT), 41);
        assert_eq!(p2(&INPUT), 6);
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("p1 {}", p1(&input));
    println!("p2 {}", p2(&input));
    Ok(())
}
