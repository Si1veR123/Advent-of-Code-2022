# https://adventofcode.com/2022/day/6

with open("../data/6.txt") as file:
    buffer = file.read()

def find_non_repeated(length, buf):
    for n in range(len(buf)-(length-1)):
        if len(list(set(buf[n:n+length]))) == length:
            return n+length

print("Part 1")
print(find_non_repeated(4, buffer))
print("Part 2")
print(find_non_repeated(14, buffer))
