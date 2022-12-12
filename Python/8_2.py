# https://adventofcode.com/2022/day/8

from itertools import chain
import importlib
a = importlib.__import__("8")
transpose = a.transpose

def count_binary_ones(binary):
    return sum([int(i) for i in list(f"{binary:08b}")])

def row_visibility_score(row):
    right_visibility_score = []
    left_visibility_score = []

    # iterate over right-most trees for each tree and calculate visible trees 
    for position, elem in enumerate(row):
        if position == 0 or position == len(row)-1:
            right_visibility_score.append(0)
            continue

        for forward_pos, forward_height in enumerate(row[position+1:]):
            if forward_height >= elem:
                right_visibility_score.append(forward_pos+1)
                break
        else:
            right_visibility_score.append(len(row)-position-1)
    
    # iterate over left-most trees for each tree and calculate visible trees 
    for position, elem in enumerate(row):
        if position == 0 or position == len(row)-1:
            left_visibility_score.append(0)
            continue
        
        for forward_pos, forward_height in enumerate(row[:position][::-1]):
            if forward_height >= elem:
                left_visibility_score.append(forward_pos+1)
                break
        else:
            left_visibility_score.append(position)

    horizontal_scores = [x*y for x, y in zip(right_visibility_score, left_visibility_score)]

    return horizontal_scores

def element_wise_matrix_multiplication(matrix1, matrix2):
    return [[x*y for x, y in zip(row1, row2)] for row1, row2 in zip(matrix1, matrix2)]


# read as a 2d list
with open("../data/8.txt") as file:
    data = file.read()
height_matrix = [[int(char) for char in line] for line in data.splitlines()]


horizontal_scores = [row_visibility_score(row) for row in height_matrix]
vertical_scores = transpose([row_visibility_score(row) for row in transpose(height_matrix)])
overall_scores = element_wise_matrix_multiplication(horizontal_scores, vertical_scores)
print("Part 2")
print(max(chain.from_iterable(overall_scores)))
