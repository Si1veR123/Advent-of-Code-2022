# https://adventofcode.com/2022/day/8

# Includes 2 different methods
# Method 2 seems to be fastest (using timeit)

"""
METHOD 1 ALGORITHM
Overview:
Given a row of heights, return a binary number representing the visible trees
e.g. if 1st 2nd and 5th trees are visible, will return 11001 (25)

1) find max in row
2) encode the position of max in binary
      If multiple occurences of maximum, and looking from both directions, encode the first and last occurence
      Else, (looking from left or right) encode the first (looking from left) or last (looking from right) occurence
3) get left sublist of first max occurence and right sublist of last max occurence
4) repeat from 1 with new sublist, taking into account the direction, until empty


Note: when combining the encoded positions from recursions using bitwise OR, bits must be shifted to account for relative indices

Example with a recursion depth of 2
(Main call)
[1, 2, 3, 2, 2] direction=both
-> binary 00100 (before combining)

(Left recursion call)
[1, 2] direction=left
-> binary 11

(Right recursion call)
[2, 2] direction=right
-> binary 01

(Combining)
Shift left call (11) by << 3 (len(row) - first occurence index)
-> 11000

Right call doesnt need to be shifted as it is relative to right side already

combined: shifted left OR right OR main
-> 11000 OR 00001 OR 00100 = 11101
"""

import typing

MATRIX_TYPE = typing.Iterable[typing.Iterable[int]]

def visible_in_row_method_one(row: typing.List[int], visible_direction=0) -> int:
    # direction (looking from left/right/both) is 0, 1 or 2 (0=both, 1=left, 2=right)
    if len(row) == 0:
        return 0

    visible_as_binary = 0
    max_height = max(row)

    # first occurence of the max value
    first_occurence = row.index(max_height)
    # last occurence of the max value
    last_occurence = len(row) - 1 - row[::-1].index(max_height)

    # any direction or left direction
    if visible_direction in (0, 1):
        # encode the first occurence of max value as binary
        visible_as_binary |= (1 << len(row) - 1 - first_occurence)

        # calculate the visibility of the sublist relative to the current binary
        relative_visible_binary = visible_in_row_method_one(row[:first_occurence], 1)
        absolute_visible_binary = relative_visible_binary << (len(row) - first_occurence)

        # bitwise OR with the sublist visiblity
        visible_as_binary |= absolute_visible_binary

    # any direction or right direction
    if visible_direction in (0, 2):
        # encode the last occurence of max value as binary
        visible_as_binary |= (1 << len(row) - 1 - last_occurence)

        absolute_visible_binary = visible_in_row_method_one(row[last_occurence+1:], 2)

        # bitwise OR with the sublist visiblity
        visible_as_binary |= absolute_visible_binary

    return visible_as_binary

def visible_in_row_method_2(row):
    binary_encoding = 0

    current_max_height = -1
    for index, tree in enumerate(row):
        if tree > current_max_height:
            current_max_height = tree
            binary_encoding |= (1 << (len(row)-index-1))
    
    current_max_height = -1
    for index, tree in enumerate(row[::-1]):
        if tree > current_max_height:
            current_max_height = tree
            binary_encoding |= (1 << index)
    
    return binary_encoding

def transpose(matrix: MATRIX_TYPE) -> MATRIX_TYPE:
    return list(zip(*matrix))


def run_method(matrix: MATRIX_TYPE, method):
    row_visiblities = [method(row) for row in matrix]

    # find column visibilities by transposing
    # calculating row visibilities
    # representing each row as a list of "1" or "0"
    # transpose again
    # combine list of "1" and "0" to a binary string, then convert to integer
    col_visibilities = transpose([list(f"{method(row):08b}") for row in transpose(matrix)])
    col_visibilities = [int("".join(i), 2) for i in col_visibilities]

    # bitwise OR the row and col visibilities to generate overall visibilities
    overall_visibilities = [f"{(row | col):08b}" for row, col in zip(row_visiblities, col_visibilities)]
    return overall_visibilities

if __name__ == "__main__":
    import timeit

    # read as a 2d list
    with open("../data/8.txt") as file:
        data = file.read()
    height_matrix = [[int(char) for char in line] for line in data.splitlines()]
    
    print("Timing method one")
    print(timeit.timeit("run_method(height_matrix, visible_in_row)", setup="from __main__ import run_method, height_matrix, visible_in_row", number=1000))
    print("Timing method two")
    print(timeit.timeit("run_method(height_matrix, visible_in_row_method_2)", setup="from __main__ import run_method, height_matrix, visible_in_row_method_2", number=1000))

    overall_visibilities = run_method(height_matrix, visible_in_row_method_one)

    visible = sum([sum([int(bit) for bit in row]) for row in overall_visibilities])
    print("Part 1")
    print(visible)
