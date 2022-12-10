# https://adventofcode.com/2022/day/5

from queue import LifoQueue

class State:
    def __init__(self, initial) -> None:
        self.stacks = [LifoQueue() for _ in range(len(initial))]

        for stack_values, stack_q in zip(initial, self.stacks):
            [stack_q.put(v) for v in stack_values[::-1]]

    def move(self, n, from_stack, to_stack):
        for _ in range(n):
            self.stacks[to_stack-1].put(self.stacks[from_stack-1].get())

    def move_multiple(self, n, from_stack, to_stack):
        holding_crates = []
        for _ in range(n):
            holding_crates.append(self.stacks[from_stack-1].get())
        for crate in holding_crates[::-1]:
            self.stacks[to_stack-1].put(crate)

initial = [
    ["N", "V", "C", "S"],
    ["S", "N", "H", "J", "M", "Z"],
    ["D", "N", "J", "G", "T", "C", "M"],
    ["M", "R", "W", "J", "F", "D", "T"],
    ["H", "F", "P"],
    ["J", "H", "Z", "T", "C"],
    ["Z", "L", "S", "F", "Q", "R", "P", "D"],
    ["W", "P", "F", "D", "H", "L", "S", "C"],
    ["Z", "G", "N", "F", "P", "M", "S", "D"]
]

state = State(initial)

with open("../data/5.txt") as file:
    lines = file.read().splitlines()[10:]

parsed_args = [(int(line.split()[1]), int(line.split()[3]), int(line.split()[5])) for line in lines]
for args in parsed_args:
    state.move(*args)

print("Part 1")
for stack in state.stacks:
    print(stack.get(), end="")

print("\nPart 2")
state = State(initial)

for args in parsed_args:
    state.move_multiple(*args)

for stack in state.stacks:
    print(stack.get(), end="")
