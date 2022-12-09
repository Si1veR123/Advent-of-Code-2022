# https://adventofcode.com/2022/day/1

with open("../data/1.txt", "r") as file:
    lines = file.readlines()

sums = []
running_sum = 0
for line in lines:
    if line == "\n":
        sums.append(running_sum)
        running_sum = 0
    else:
        running_sum += int(line)

print("Part 1")
print(max(sums))
print("Part 2")
print(sum(sorted(sums)[-3:]))
