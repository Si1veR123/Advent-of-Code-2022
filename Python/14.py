

class SandSim:
    def __init__(self, rock_grid):
        self.rock_grid = rock_grid
        self.rested_sand = []
        self.sand_source = (500, 0)

    @classmethod
    def from_line_coords(cls, lines):
        rock_coords = []

        for line in lines:
            for i in range(len(line)-1):
                start = line[i]
                end = line[i+1]
                if start[0] == end[0]:
                    from_and_to = (start[1], end[1])
                    for y in range(min(from_and_to), max(from_and_to)+1):
                        rock_coords.append((start[0], y))
                else:
                    from_and_to = (start[0], end[0])
                    for x in range(min(from_and_to), max(from_and_to)+1):
                        rock_coords.append((x, start[1]))

        return cls(rock_coords)
    
    def get_grid_size(self):
        maxi_x = 0
        maxi_y = 0
        mini_x = float("inf")
        mini_y = float("inf")

        for rock in self.rock_grid + self.rested_sand:
            maxi_x = max(maxi_x, rock[0])
            maxi_y = max(maxi_y, rock[1])

            mini_x = min(mini_x, rock[0])
            mini_y = min(mini_y, rock[1])
        return (mini_x, mini_y), (maxi_x, maxi_y)
    
    def print_grid(self):
        mini, maxi = self.get_grid_size()
        for y in range(mini[1], maxi[1]+1):
            for x in range(mini[0], maxi[0]+1):
                if (x, y) in self.rock_grid:
                    print("#", end="")
                elif (x, y) in self.rested_sand:
                    print("O", end="")
                elif (x, y) == (self.sand_source[0], mini[1]):
                    print("x", end="")
                else:
                    print(".", end="")
            print("")

    def spawn_sand(self):
        sand_pos = self.sand_source
        _, max_bounds = self.get_grid_size()
        while True:
            # down, left + down, right + down, or lay at rest
            blocked = self.rested_sand + self.rock_grid

            next_down_pos = (sand_pos[0], sand_pos[1]+1)
            if next_down_pos in blocked:
                left_diag_pos = (sand_pos[0]-1, sand_pos[1]+1)
                if left_diag_pos in blocked:
                    right_diag_pos = (sand_pos[0]+1, sand_pos[1]+1)
                    if right_diag_pos in blocked:
                        self.rested_sand.append(sand_pos)
                        if sand_pos == self.sand_source:
                            return True
                        return False
                    else:
                        sand_pos = right_diag_pos
                else:
                    sand_pos = left_diag_pos
            else:
                if sand_pos[1] >= max_bounds[1]:
                    return True
                sand_pos = next_down_pos
    
    def simulate(self):
        self.rested_sand = []
        ended = False
        while not ended:
            ended = self.spawn_sand()
        return len(self.rested_sand)


with open("../data/14.txt") as file:
    lines = file.read().splitlines()

rock_lines = [[[int(c) for c in coord.split(",")] for coord in line.split(" -> ")] for line in lines]
sandsim = SandSim.from_line_coords(rock_lines)

print(sandsim.simulate())

# could use math to do this but brute force instead because its 1am
_, (_, max_floor_height) = sandsim.get_grid_size()
line = SandSim.from_line_coords([[[-9999, max_floor_height+2], [9999, max_floor_height+2]]]).rock_grid
sandsim.rock_grid += line
print(sandsim.simulate())
