# https://adventofcode.com/2022/day/4

with open("../data/4.txt") as file:
    lines = file.read().splitlines()

# parse all lines and apply func to check if ranges overlap
eval_overlaps = lambda func: sum([int(func(*[range( int(r.split("-")[0]), int(r.split("-")[1]) ) for r in line.split(",")])) for line in lines])
range_overlap_complete = lambda x, y: (x.start <= y.start and x.stop >= y.stop) or (y.start <= x.start and y.stop >= x.stop)
range_overlap_any = lambda x, y: (x.start <= y.start <= x.stop and y.stop >= x.stop) or (y.start <= x.start <= y.stop and x.stop >= y.stop) or range_overlap_complete(x, y)

print("Part 1\n", eval_overlaps(range_overlap_complete))
print("Part 2\n", eval_overlaps(range_overlap_any))
