# Day 3: Gear Ratios

You and the Elf eventually reach a gondola lift station; he says the gondola lift will
take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not
moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise.
"Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still
be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but
nobody can figure out which one. If you can add up all the part numbers in the engine
schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the
engine. There are lots of numbers and symbols you don't really understand, but
apparently any number adjacent to a symbol, even diagonally, is a "part number" and
should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

```
0467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, two numbers are not part numbers because they are not adjacent to a
symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a
symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the
part numbers in the engine schematic?

## Part 1

## Solution

```python

import re
import string

with open('input.txt', 'r') as f:
    lines = f.readlines()

# extract symbols from input excluding periods
symbols = set(re.sub(r'[\d\n.]', '', ''.join(lines)))

# iterate over lines finding each number, along with their corresponding start and stop
# index via `.span()` to be used in finding adjacent symbols
numbers = []
for index, line in enumerate(lines):
    for match in re.finditer(r'(\w+)', line):
        start, stop = match.span()
        adj_start = start if start == 0 else start - 1
        adj_stop = stop if stop == len(line) else stop + 1

        # adjacent characters in previous line
        line_prev = '' if index == 0 else lines[index - 1][adj_start:adj_stop]

        # adjacent characters in current line
        line_curr = lines[index][adj_start:adj_stop]

        # adjacent characters in following line
        line_next = '' if index + 1 == len(lines) else lines[index + 1][adj_start:adj_stop]

        # check if symbols exist in any adjacent characters
        if any(symbol in ''.join([line_prev, line_curr, line_next]) for symbol in symbols):
            numbers.append(int(match.group()))

sum(numbers)

# 546312

```

## Part 2

The engineer finds the missing part and installs it in the engine! As the engine springs
to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong?
Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer
answers.

Before you can explain the situation, she suggests that you look out the window. There
stands the engineer, holding a phone in one hand and waving with the other. You're going
so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear
is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the
result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the
engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

```
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
```

In this schematic, there are two gears. The first is in the top left; it has part
numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right;
its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only
adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?

### Solution

```python

# find all numbers that have an adjacent asterisk, keeping track of the index of that
# asterisk; then, find which numbers share that asterisk.

import re
import operator
import string

from functools import reduce

with open('input.txt', 'r') as f:
    lines = f.readlines()

# extract symbols from input excluding periods
symbols = set(re.sub(r'[\d\n.]', '', ''.join(lines)))

def _asterisk_indexes(line):
    """finds x coordinate of asterisks in list.
    """
    return [index for index, char in enumerate(line) if char == '*']

# iterate over lines finding each number, along with their corresponding start and stop
# index via `.span()` to be used in finding adjacent symbols.
nums: dict = {}
for index, line in enumerate(lines):
    for match in re.finditer(r'(\w+)', line):
        start, stop = match.span()
        adj_start = start if start == 0 else start - 1
        adj_stop = stop if stop == len(line) else stop + 1

        # Store asterisk coordinates in dictionary, and append numbers to that
        # dictionary key. The dictionary values that have more than one number will be
        # used in the final calculation.

        line_prev = '' if index == 0 else lines[index - 1][adj_start:adj_stop]
        if ixs := _asterisk_indexes(line_prev):
            for x in ixs:
                coords = (x + adj_start, index - 1)
                nums[coords] = nums.get(coords, []) + [int(match.group())]

        line_curr = lines[index][adj_start:adj_stop]
        if ixs := _asterisk_indexes(line_curr):
            for x in ixs:
                coords = (x + adj_start, index)
                nums[coords] = nums.get(coords, []) + [int(match.group())]

        line_next = '' if index + 1 == len(lines) else lines[index + 1][adj_start:adj_stop]
        if ixs := _asterisk_indexes(line_next):
            for x in ixs:
                coords = (x + adj_start, index + 1)
                nums[coords] = nums.get(coords, []) + [int(match.group())]


tally = 0
for coordinate, numbers in nums.items():
    if len(numbers) > 1:
        tally += reduce(operator.mul, numbers)

# 87449461

```
