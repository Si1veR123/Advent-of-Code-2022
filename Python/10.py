# https://adventofcode.com/2022/day/10

with open("../data/10.txt") as file:
    data = file.read().splitlines()

signals = []
current_cycle = 1  # 1 = during first cycle, 2 = during second cycle etc.
x = 1
for instruction in data:
    if instruction == "noop":
        signals.append(x*current_cycle)
        current_cycle += 1

    elif instruction.startswith("addx"):
        signals.append(x*current_cycle)
        current_cycle += 1
        signals.append(x*current_cycle)
        current_cycle += 1

        amount = int(instruction[5:])
        x += amount

print("Part 1")
print(signals[19] + signals[59] + signals[99] + signals[139] + signals[179] + signals[219])

print("Part 2")

def what_to_print(x, cycle):
    newline = cycle % 40 == 0

    # middle
    sprite_binary_mask = (1 << (39-x))
    # left
    sprite_binary_mask |= (1 << (40-x))
    # right (abs to prevent -1)
    sprite_binary_mask |= (1 << abs(38-x))

    # first cycle is 1, so subtract 1
    x_pos = (cycle-1) % 40

    cycle_binary_mask = (1 << (39-x_pos))

    # copied and pasted these ascii symbols to make it clearer
    if sprite_binary_mask & cycle_binary_mask:
        return "█\n" if newline else "█"
    return " \n" if newline else " "

current_cycle = 1
x = 1
for instruction in data:
    if instruction == "noop":
        print(what_to_print(x, current_cycle), end="")
        current_cycle += 1

    elif instruction.startswith("addx"):
        print(what_to_print(x, current_cycle), end="")
        current_cycle += 1
        print(what_to_print(x, current_cycle), end="")
        current_cycle += 1

        amount = int(instruction[5:])
        x += amount
