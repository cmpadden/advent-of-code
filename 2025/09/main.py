"""
--- Day 9: Movie Theater ---

You slide down the firepole in the corner of the playground and land in the North Pole base movie theater!

The movie theater has a big tile floor with an interesting pattern. Elves here are redecorating the theater by switching out some of the square tiles in the big grid they form. Some of the tiles are red; the Elves would like to find the largest rectangle that uses red tiles for two of its opposite corners. They even have a list of where the red tiles are located in the grid (your puzzle input).

For example:

7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3

Showing red tiles as # and other tiles as ., the above arrangement of red tiles would look like this:

..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............

You can choose any two red tiles as the opposite corners of your rectangle; your goal is to find the largest rectangle possible.

For example, you could make a rectangle (shown as O) with an area of 24 between 2,5 and 9,7:

..............
.......#...#..
..............
..#....#......
..............
..OOOOOOOO....
..OOOOOOOO....
..OOOOOOOO.#..
..............

Or, you could make a rectangle with area 35 between 7,1 and 11,7:

..............
.......OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
..#....OOOOO..
.......OOOOO..
.......OOOOO..
..............

You could even make a thin rectangle with an area of only 6 between 7,3 and 2,3:

..............
.......#...#..
..............
..OOOOOO......
..............
..#......#....
..............
.........#.#..
..............

Ultimately, the largest rectangle you can make in this example has area 50. One way to do this is between 2,5 and 11,1:

..............
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..OOOOOOOOOO..
..............
.........#.#..
..............

Using two red tiles as opposite corners, what is the largest area of any rectangle you can make?

"""

from functools import lru_cache

C = [(*map(int, l.split(",")),) for l in open("2025/09/input.txt")]
print(max((abs(a - c) + 1) * (abs(b - d) + 1) for a, b in C for c, d in C))

"""
--- Part Two ---

The Elves just remembered: they can only switch out tiles that are red or green. So, your rectangle can only include red or green tiles.

In your list, every red tile is connected to the red tile before and after it by a straight line of green tiles. The list wraps, so the first red tile is also connected to the last red tile. Tiles that are adjacent in your list will always be on either the same row or the same column.

Using the same example as before, the tiles marked X would be green:

..............
.......#XXX#..
.......X...X..
..#XXXX#...X..
..X........X..
..#XXXXXX#.X..
.........X.X..
.........#X#..
..............

In addition, all of the tiles inside this loop of red and green tiles are also green. So, in this example, these are the green tiles:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............

The remaining tiles are never red nor green.

The rectangle you choose still must have red tiles in opposite corners, but any other tiles it includes must now be red or green. This significantly limits your options.

For example, you could make a rectangle out of red and green tiles with an area of 15 between 7,3 and 11,1:

..............
.......OOOOO..
.......OOOOO..
..#XXXXOOOOO..
..XXXXXXXXXX..
..#XXXXXX#XX..
.........XXX..
.........#X#..
..............

Or, you could make a thin rectangle with an area of 3 between 9,7 and 9,5:

..............
.......#XXX#..
.......XXXXX..
..#XXXX#XXXX..
..XXXXXXXXXX..
..#XXXXXXOXX..
.........OXX..
.........OX#..
..............

The largest rectangle you can make in this example using only red and green tiles has area 24. One way to do this is between 9,5 and 2,3:

..............
.......#XXX#..
.......XXXXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
..OOOOOOOOXX..
.........XXX..
.........#X#..
..............

Using two red tiles as opposite corners, what is the largest area of any rectangle you can make using only red and green tiles?
"""

R = set(C)


@lru_cache(maxsize=None)
def B(x, y):
    for i in range(len(C)):
        x1, y1 = C[i]
        x2, y2 = C[(i + 1) % len(C)]
        if x1 == x2 == x and min(y1, y2) <= y <= max(y1, y2):
            return 1
        if y1 == y2 == y and min(x1, x2) <= x <= max(x1, x2):
            return 1
    return 0


@lru_cache(maxsize=None)
def I(x, y):
    c = 0
    p1x, p1y = C[0]
    for j in range(1, len(C) + 1):
        p2x, p2y = C[j % len(C)]
        if (
            min(p1y, p2y) < y <= max(p1y, p2y)
            and x <= max(p1x, p2x)
            and (p1y == p2y or x <= (y - p1y) * (p2x - p1x) / (p2y - p1y) + p1x)
        ):
            c = 1 - c
        p1x, p1y = p2x, p2y
    return c


def V(a, b, c, d):
    minx, maxx = min(a, c), max(a, c)
    miny, maxy = min(b, d), max(b, d)
    for x in range(minx, maxx + 1):
        for y in [miny, maxy]:
            if (x, y) not in R and not B(x, y) and not I(x, y):
                return 0
    for y in range(miny + 1, maxy):
        for x in [minx, maxx]:
            if (x, y) not in R and not B(x, y) and not I(x, y):
                return 0
    return 1


print(
    max(
        (abs(a - c) + 1) * (abs(b - d) + 1)
        for i, (a, b) in enumerate(C)
        for j, (c, d) in enumerate(C[i + 1 :], i + 1)
        if V(a, b, c, d)
    )
)
