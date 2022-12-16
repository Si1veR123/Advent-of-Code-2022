# https://adventofcode.com/2022/day/13

import json
import time

def compare_value(val1, val2):
    types = (isinstance(val1, int), isinstance(val2, int))
    if all(types):
        if val1 == val2:
            return None
        return val1 < val2

    if any(types):
        if types[0]:
            val = compare_value([val1], val2)
            return val
        else:
            val = compare_value(val1, [val2])
            return val
            
    # both lists
    for i in range(max(len(val1), len(val2))):
        try:
            left_list = val1[i]
        except IndexError:
            return True
        
        try:
            right_list = val2[i]
        except IndexError:
            return False
        
        result = compare_value(left_list, right_list)
        if result is None:
            continue
        else:
            return result

start = time.time()

with open("../data/13.txt") as file:
    data = file.read().splitlines()

pairs = []
current_pair = ()
for line in data:
    if line == "":
        pairs.append(current_pair)
        current_pair = ()
        continue

    current_pair += (json.loads(line),)
pairs.append(current_pair)

count = 0
for n, pair in enumerate(pairs):
    if compare_value(*pair):
        count += (n+1)
print("Part 1")
print(count)

all_packets = list(map(lambda x: json.loads(x), filter(lambda x: len(x)>0, data)))
all_packets.append([[2]])
all_packets.append([[6]])

# bubble sort
ordered = False
while not ordered:
    ordered = True
    for i in range(len(all_packets)-1):
        first_val = all_packets[i]
        next_val = all_packets[i+1]
        if not compare_value(first_val, next_val):
            all_packets.remove(first_val)
            all_packets.insert(i+1, first_val)
            ordered = False
        
packet_loc_1 = all_packets.index([[2]])+1
packet_loc_2 = all_packets.index([[6]])+1
print("Part 2")
print(packet_loc_1*packet_loc_2)

print("Took:", time.time() - start)
