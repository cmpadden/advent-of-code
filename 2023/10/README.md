# Day 10: Pipe Maze ---

You use the hang glider to ride the hot air from Desert Island all the way up to the
floating metal island. This island is surprisingly cold and there definitely aren't any
thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals. However, you do
occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent
direction; maybe you can find someone at the hot springs and ask them where the
desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As you stop
to admire some metal grass, you notice something metallic scurry away in your peripheral
vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you
want a better look, you'll need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely
packed with pipes; it was hard to tell at first because they're the same metallic silver
color as the "ground". You make a quick sketch of all of the surface pipes you can see
(your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

 - | is a vertical pipe connecting north and south.
 - - is a horizontal pipe connecting east and west.
 - L is a 90-degree bend connecting north and east.
 - J is a 90-degree bend connecting north and west.
 - 7 is a 90-degree bend connecting south and west.
 - F is a 90-degree bend connecting south and east.
 - . is ground; there is no pipe in this tile.
 - S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

Based on the acoustics of the animal's scurrying, you're confident the pipe that
contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

```
.....
.F-7.
.|.|.
.L-J.
.....
```

If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

```
.....
.S-7.
.|.|.
.L-J.
.....
```

In the above diagram, the S tile is still a 90-degree F bend: you can tell because of
how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This sketch
shows the same loop as above:

```
-L|F7
7S-7|
L|7||
-L-J|
L|-JF
```

In the above diagram, you can still figure out which pipes form the main loop: they're
the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and
so on. Every pipe in the main loop connects to its two neighbors (including S, which
will have exactly two pipes connecting to it, and which is assumed to connect back to
those two pipes).

Here is a sketch that contains a slightly more complex main loop:

```
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
```

Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

```
7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
```

If you want to get out ahead of the animal, you should find the tile in the loop that is
farthest from the starting position. Because the animal is in the pipe, it doesn't make
sense to measure this by direct distance. Instead, you need to find the tile that would
take the longest number of steps along the loop to reach from the starting point -
regardless of which way around the loop the animal went.

In the first example with the square loop:

```
.....
.S-7.
.|.|.
.L-J.
.....
```

You can count the distance each tile in the loop is from the starting point like this:

```
.....
.012.
.1.3.
.234.
.....
```

In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

```
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
```

Here are the distances for each tile on that loop:

```
..45.
.236.
01.78
14567
23...
```

Find the single giant loop starting at S. How many steps along the loop does it take to
get from the starting position to the point farthest from the starting position?

## Part 1

### Solution

```python

from dataclasses import dataclass
from typing import Optional, Tuple


def _start(grid: list[str]):
    for y, r in enumerate(grid):
        for x, c in enumerate(r):
            if c == 'S':
                return (x, y)


def next_coordinate(
    grid: list[list[str]],
    previous: Tuple[[int, int]],
    current: Tuple[[int, int]]
) -> Tuple[[int, int]]:
    px, py = previous
    x, y = current
    tile = grid[y][x]
    print(f"{previous=}, {current=}, {tile=}")
    # | is a vertical pipe connecting north and south.
    if tile == "|":
        if y > py:
            print((x, y), (x, y + 1))
            return (x, y + 1)
        else:
            return (x, y - 1)
    # - is a horizontal pipe connecting east and west.
    elif tile == "-":
        if x > px:
            return (x + 1, y)
        else:
            return (x - 1, y)
    # L is a 90-degree bend connecting north and east.
    elif tile == "L":
        if x < px:  # LX
            return (x, y - 1)
        else:
            return (x + 1, y)
    # J is a 90-degree bend connecting north and west.
    elif tile == "J":
        if x > px:
            return (x, y - 1)
        else:
            return (x - 1, y)
    # 7 is a 90-degree bend connecting south and west.
    elif tile == "7":
        if x > px:
            return (x, y + 1)
        else:
            return (x - 1, y)
    # F is a 90-degree bend connecting south and east.
    elif tile == "F":
        if x < px:  # FX
            return (x, y + 1)
        else:
            return (x + 1, y)
    else:
        raise Exception("Unsupported tile")


# ..F7.
# .FJ|.
# SJ.L7
# |F--J
# LJ...

assert next_coordinate(grid, (0, 2), (0, 3)) == (0, 4)  # S -> | -> L

assert next_coordinate(grid, (0, 3), (0, 4)) == (1, 4)  # | -> L -> J

assert next_coordinate(grid, (0, 4), (1, 4)) == (1, 3)  # L -> J -> F

assert next_coordinate(grid, (1, 4), (1, 3)) == (2, 3)  # J -> F -> -

assert next_coordinate(grid, (1, 3), (2, 3)) == (3, 3)  # F -> - -> -

assert next_coordinate(grid, (2, 3), (3, 3)) == (4, 3)  # - -> - -> J

assert next_coordinate(grid, (3, 3), (4, 3)) == (4, 2)  # - -> J -> 7

assert next_coordinate(grid, (4, 3), (4, 2)) == (3, 2)  # J -> 7 -> L

assert next_coordinate(grid, (4, 2), (3, 2)) == (3, 1)  # 7 -> L -> |

assert next_coordinate(grid, (3, 2), (3, 1)) == (3, 0)  # L -> | -> 7

assert next_coordinate(grid, (3, 1), (3, 0)) == (2, 0)  # | -> 7 -> F

assert next_coordinate(grid, (3, 0), (2, 0)) == (2, 1)  # 7 -> F -> J

assert next_coordinate(grid, (2, 0), (2, 1)) == (1, 1)  # F -> J -> F

assert next_coordinate(grid, (2, 1), (1, 1)) == (1, 2)  # J -> F -> J

assert next_coordinate(grid, (1, 1), (1, 2)) == (0, 2)  # F -> J -> S


input = """\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"""

grid: list[str] = [r for r in open('input.txt').read().split('\n') if r]

previous = _start(grid)

current = previous[0], previous[1] + 1

index = 1
coords = [previous, current]
while grid[coords[index][1]][coords[index][0]] != 'S':
    coords.append(
        next_coordinate(grid, coords[index - 1], coords[index])
    )
    index += 1

len(coords) // 2

```


