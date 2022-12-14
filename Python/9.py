# https://adventofcode.com/2022/day/9
import time

class Rope:
    def __init__(self):
        self.head = (0, 0)
        self.tail = (0, 0)
        self.tail_positions = [(0, 0)]
    
    def adjacent(self):
        return (abs(self.head[0]-self.tail[0]) <= 1) and (abs(self.head[1]-self.tail[1]) <= 1)

    def move_head(self, direction, n):
        direction = direction.lower()
        for _ in range(n):
            start_head = (self.head[0], self.head[1])
            if direction == "u":
                self.head = (self.head[0], self.head[1]+1)
            elif direction == "d":
                self.head = (self.head[0], self.head[1]-1)
            elif direction == "l":
                self.head = (self.head[0]-1, self.head[1])
            elif direction == "r":
                self.head = (self.head[0]+1, self.head[1])
            
            if not self.adjacent():
                self.tail = start_head
                if self.tail not in self.tail_positions:
                    self.tail_positions.append(self.tail)

start = time.time()

with open("../data/9.txt") as file:
    data = file.read().splitlines()

rope = Rope()
for line in data:
    direction, n = line.split()
    rope.move_head(direction, int(n))
print("Part 1")
print(len(rope.tail_positions))

# ========= Part 2 ==========

class Knot:
    def __init__(self, child):
        self.location = (0, 0)
        self.child = child
        self.location_cache = [(0, 0)]
    
    def _adjacent_to_child(self):
        if self.child is None:
            return True
        return (abs(self.location[0]-self.child.location[0]) <= 1) and (abs(self.location[1]-self.child.location[1]) <= 1)

    def _cache_loc(self):
        if self.location not in self.location_cache:
            self.location_cache.append(self.location)

    def _calculate_next_move(self):
        if self.location == self.child.location:
            return (0, 0)

        elif self.location[0] == self.child.location[0]:
            # same column
            
            if self.location[1] > self.child.location[1]:
                return (0, 1)
            elif self.location[1] < self.child.location[1]:
                return (0, -1)
        
        elif self.location[1] == self.child.location[1]:
            # same row

            if self.location[0] > self.child.location[0]:
                return (1, 0)
            elif self.location[0] < self.child.location[0]:
                return (-1, 0)
        
        else:
            # diagonal
            # get vector from child to self
            # make each component of vector -1 or 1
            vector = (self.location[0]-self.child.location[0], self.location[1]-self.child.location[1])
            return (-1 if vector[0] < 0 else 1, -1 if vector[1] < 0 else 1)
        

    def _move_by(self, rel_change):
        self.location = (self.location[0]+rel_change[0], self.location[1]+rel_change[1])

    def _move_catch_up(self, rel_change):
        self._move_by(rel_change)
        self._cache_loc()

        if not self._adjacent_to_child():
            next_move = self._calculate_next_move()
            self.child._move_catch_up(next_move)

    def move_head(self, direction, n):
        direction = direction.lower()
        for _ in range(n):
            if direction == "u":
                rel_location = (0, 1)
            elif direction == "d":
                rel_location = (0, -1)
            elif direction == "l":
                rel_location = (-1, 0)
            elif direction == "r":
                rel_location = (1, 0)
            self._move_catch_up(rel_location)
    
    def __repr__(self) -> str:
        return f"Location: {str(self.location)}, HasChild: {self.child is not None}"


rope = []
child = None
for n in range(10):
    child = Knot(child)
    rope.append(child)

for line in data:
    direction, n = line.split()
    rope[-1].move_head(direction, int(n))

print("Part 2")
print(len(rope[0].location_cache))
print("Took: ", time.time()-start)
