# https://adventofcode.com/2022/day/11

from copy import deepcopy
import time

class Monkey:
    def __init__(self, items, operation, divis_test, truthy_monkey, falsy_monkey, worry_level_reduction):
        self.items = items
        self.operation = operation
        self.divisible_test = divis_test
        self.truthy_monkey_id = truthy_monkey
        self.falsy_monkey_id = falsy_monkey

        self.inspect_count = 0

        self.worry_level_reduction = worry_level_reduction
    
    def __repr__(self):
        return f"Monkey(Items: {self.items}, Operation: {self.operation}, Divisibility test: {self.divisible_test}, Truthy monkey: {self.truthy_monkey_id}, Falsy monkey: {self.falsy_monkey_id})"

    def receive_item(self, worry_level):
        self.items.append(worry_level)

    def inspect_item(self, index, common_multiple):
        self.inspect_count += 1

        # inspect item at index in self.items
        # return: id of monkey to throw to
        old = self.items[index]

        # unsafe to use
        worry_level = eval(self.operation)
        bored_level = int(worry_level/self.worry_level_reduction) % common_multiple

        # update the item with the new 'worry level'
        self.items[index] = bored_level

        if (bored_level % self.divisible_test) == 0:
            return self.truthy_monkey_id
        return self.falsy_monkey_id

    def execute_turn(self, common_multiple):
        receiving_monkeys = []
        for i in range(len(self.items)):
            receiving_monkeys.append(self.inspect_item(i, common_multiple))
        to_return = (receiving_monkeys, deepcopy(self.items))
        self.items.clear()
        return to_return


def parse_monkeys(data, worry_level_reduction):
    lines = [line.strip() for line in data.splitlines()]

    monkey_lines_data = []
    current_monkey = []
    for line in lines:
        if line == "":
            monkey_lines_data.append(current_monkey)
            current_monkey = []
        else:
            current_monkey.append(line)
    monkey_lines_data.append(current_monkey)  # last monkey

    monkeys = []
    for monkey_data in monkey_lines_data:
        starting_items_string = monkey_data[1][16:]
        start_items_parsed = [int(item) for item in starting_items_string.split(",")]
        operation_string = monkey_data[2][17:]
        divisible_by_test = int(monkey_data[3][19:])
        truthy_monkey = int(monkey_data[4][25])
        falsy_monkey = int(monkey_data[5][26])

        monkeys.append(Monkey(start_items_parsed, operation_string, divisible_by_test, truthy_monkey, falsy_monkey, worry_level_reduction))
    return monkeys

def simulate_monkeys(monkeys, rounds):
    # common multiple is used to reduce worry levels in part 2
    common_multiple = 1
    for monkey in monkeys:
        common_multiple *= monkey.divisible_test

    for n in range(rounds):
        for monkey in monkeys:
            receiving_monkeys, items = monkey.execute_turn(common_multiple)
            
            for receiving_monkey, item in zip(receiving_monkeys, items):
                monkey_obj: Monkey = monkeys[receiving_monkey]
                monkey_obj.receive_item(item)

    monkeys_sorted = sorted(monkeys, key=lambda x: x.inspect_count, reverse=True)
    monkey_business = monkeys_sorted[0].inspect_count * monkeys_sorted[1].inspect_count
    return monkey_business

start = time.time()

with open("../data/11.txt") as file:
    data = file.read()

monkeys = parse_monkeys(data, 3)
monkey_business = simulate_monkeys(monkeys, 20)
print("Part 1")
print("Monkey business:", monkey_business)


monkeys = parse_monkeys(data, 1)
monkey_business = simulate_monkeys(monkeys, 10000)
print("Part 2")
print("Monkey business:", monkey_business)

print("Took: ", time.time() - start)
