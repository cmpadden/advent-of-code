# Day 11: Cosmic Expansion

You continue following signs for "Hot Springs" and eventually come across an
observatory. The Elf within turns out to be a researcher studying cosmic expansion using
the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this
research project. However, he confirms that the hot springs are the next-closest area
likely to have people; he'll even take you straight there once he's done with today's
observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single giant
image (your puzzle input). The image includes empty space (.) and galaxies (#). For
example:

```
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
```

The researcher is trying to figure out the sum of the lengths of the shortest path
between every pair of galaxies. However, there's a catch: the universe expanded in the
time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact, the
result is that any rows or columns that contain no galaxies should all actually be twice
as big.

In the above example, three columns and two rows contain no galaxies:

```
   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^
```

These rows and columns need to be twice as big; the result of cosmic expansion therefore
looks like this:

```
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
```

Equipped with this expanded universe, the shortest path between every pair of galaxies
can be found. It can help to assign every galaxy a unique number:

```
....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......
```

In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the
pair doesn't matter. For each pair, find any shortest path between the two galaxies
using only steps that move up, down, left, or right exactly one . or # at a time. (The
shortest path between two galaxies is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......

This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to
galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are
some other example shortest path lengths:

    Between galaxy 1 and galaxy 7: 15
    Between galaxy 3 and galaxy 6: 17
    Between galaxy 8 and galaxy 9: 5

In this example, after expanding the universe, the sum of the shortest path between all
36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every pair of
galaxies. What is the sum of these lengths?

To play, please identify yourself via one of these services:

## Part 1

### Solution

```python

import itertools

# Iterate through list in reverse, and insert the expanded rows into the universe; this
# is to prevent indexing issues.

def expand_rows(rows):
    num_rows = len(rows)
    num_cols = len(rows[0])
    is_row_empty = [all(c == '.' for c in row) for row in rows]
    for ix, empty in enumerate(reversed(is_row_empty)):
        if empty:
            rows.insert(num_rows - ix, ['.'] * num_cols)
    return rows


def distance(c1, c2):
    x1, y1 = c1
    x2, y2 = c2
    return abs(x2 - x1) + abs(y2 - y1)


input = """\
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."""

input = open('input.txt').read()

rows: list[list[str]] = [list(row) for row in input.split('\n') if row]

rows = expand_rows(rows)

# transpose rows to columns for easier emptiness detection, then expand the universe as
# was done above.
columns: list[list[str]] = list(map(list, zip(*rows)))

columns = expand_rows(columns)

result = list(map(list, zip(*columns)))

coordinates = []
for y, row in enumerate(result):
    for x, c in enumerate(row):
        if c == '#':
            coordinates.append((x, y))

pairs = itertools.combinations(coordinates, 2)

distances = [distance(coord1, coord2) for coord1, coord2 in pairs]

sum(distances)

# 9686930

```

## Part 2

The galaxies are much older (and thus much farther apart) than the researcher initially
estimated.

Now, instead of the expansion you did before, make each empty row or column one million
times larger. That is, each empty row should be replaced with 1000000 empty rows, and
each empty column should be replaced with 1000000 empty columns.

(In the example above, if each empty row or column were merely 10 times larger, the sum
of the shortest paths between every pair of galaxies would be 1030. If each empty row or
column were merely 100 times larger, the sum of the shortest paths between every pair of
galaxies would be 8410. However, your universe will need to expand far beyond these
values.)

Starting with the same initial image, expand the universe according to these new rules,
then find the length of the shortest path between every pair of galaxies. What is the
sum of these lengths?

### Solution

```python

# NOTE - expanding the grid, and transposing, are too inefficient as we scale the number
# of empty spaces between each start. Instead, we can keep track of the locations of
# these rows, and use them as a multiplier in our distance calculation.

import itertools

input = open('input.txt').read()

rows: list[list[str]] = [list(row) for row in input.split('\n') if row]

empty_row_ixs = set()
for y, row in enumerate(rows):
    if all([square == '.' for square in row]):
        empty_row_ixs.add(y)

empty_col_ixs = set()
for x in range(len(rows[0]) - 1):
    if all([rows[y][x] == '.' for y in range(len(rows) - 1)]):
        empty_col_ixs.add(x)

coordinates = []
for y, row in enumerate(rows):
    for x, c in enumerate(row):
        if c == '#':
            coordinates.append((x, y))

pairs = list(itertools.combinations(coordinates, 2))

# Determine the intersection of sets for the empty column / rows, and the column and
# rows between the two stars. If there are any present (via `len` then multiply by
# by our expansion multiplier.
sum = 0
for (x1, y1), (x2, y2) in pairs:
    x1, x2 = min(x1, x2), max(x1, x2)
    y1, y2 = min(y1, y2), max(y1, y2)
    sum += (x2 - x1) + len(empty_col_ixs & set(range(x1, x2 + 1))) * 999999
    sum += (y2 - y1) + len(empty_row_ixs & set(range(y1, y2 + 1))) * 999999

```
