# https://adventofcode.com/2022/day/12

import numpy as np
import time

# print all of numpy matrices for debugging
import sys
np.set_printoptions(threshold=sys.maxsize)

class HeightMapPathFinder:
    def __init__(self, heightmap: np.ndarray, start, end):
        self.heightmap = heightmap
        self.start_index = start
        self.end_index = end
    
    def print_distances_board(self, distances=None):
        for row_n, row in enumerate(self.heightmap):
            for col_n, col in enumerate(row):
                try:
                    dist = distances[(row_n, col_n)]
                except:
                    dist = -1

                if dist >= 0:
                    print("#", end="")
                else:
                    print(chr(col), end="")
            print("\n", end="")

    @staticmethod
    def find_in_matrix(matrix, num):
        occurences = []

        for row_n, row in enumerate(matrix):
            for col_n, col in enumerate(row):
                if col == num:
                    occurences.append((row_n, col_n))

        return occurences

    def _valid_step(self, from_pos, to_pos):
        # not on board
        if to_pos[0] < 0 or to_pos[1] < 0:
            return False

        # not on board
        try:
            next_val = self.heightmap[to_pos]
        except:
            return False
        
        # get 'height' value of current
        first_val = self.heightmap[from_pos]

        # move max of one up, or any down
        return (next_val - first_val) <= 1


    def _dijkstras(self):
        # find all occurences of the current distance in distance matrix
        # check if all adjacent positions are a valid move
        # if so, set adjacent positions to current distance + 1

        distances = np.empty(shape=self.heightmap.shape)
        distances.fill(-np.inf)

        distances[self.start_index] = 0

        current_max = 0
        while True:
            occurences = self.find_in_matrix(distances, current_max)

            # no route possible
            if len(occurences) == 0:
                return None

            for o in occurences:
                for surrounding_pos in [(o[0]+1, o[1]), (o[0]-1, o[1]), (o[0], o[1]+1), (o[0], o[1]-1)]:
                    if self._valid_step(o, surrounding_pos) and distances[surrounding_pos] < 0:
                        if surrounding_pos == self.end_index:
                            return current_max+1
                        distances[surrounding_pos] = current_max+1
            
            current_max += 1

            # self.print_distances_board(distances)
            # input()


    def dijkstras(self):
        return self._dijkstras()

start = time.time()

with open("../data/12.txt") as file:
    data = file.read().splitlines()

# convert letters to ascii numbers
# find S and E positions
# save them and replace with lowest point 'a' and highest point 'z'
heightmap = np.array([[ord(c) for c in row] for row in data])
start_pos = HeightMapPathFinder.find_in_matrix(heightmap, ord("S"))[0]
end_pos = HeightMapPathFinder.find_in_matrix(heightmap, ord("E"))[0]
heightmap[start_pos] = ord("a")
heightmap[end_pos] = ord("z")

print("Part 1")
pathfinder = HeightMapPathFinder( heightmap, start_pos, end_pos )
distance = pathfinder.dijkstras()
print(distance)

# brute force part 2, may be alternate methods
print("Part 2")
best_distance = np.inf
from_pos = (0, 0)

a_positions = HeightMapPathFinder.find_in_matrix(heightmap, ord("a"))
for a_pos in a_positions:
    pathfinder = HeightMapPathFinder( heightmap, a_pos, end_pos )
    distance = pathfinder.dijkstras()
    if distance is not None and distance < best_distance:
        best_distance = distance
        from_pos = a_pos
print(best_distance)

print("Took:", time.time()-start)
