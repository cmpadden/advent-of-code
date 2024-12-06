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

use std::collections::HashSet;
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

pub static INPUT1: &str = "\
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
....^.....
........#.
#.........
......#...";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert_eq!(p1(&INPUT1), 41);
        assert_eq!(p1(&INPUT2), 22);
        //assert_eq!(p2(&INPUT), 123);
    }
}

fn main() -> io::Result<()> {
    let input = fs::read_to_string("input.txt")?;
    println!("p1 {}", p1(&input));
    //println!("p2 {}", p2(&input));
    Ok(())
}
