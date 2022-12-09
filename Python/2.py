# https://adventofcode.com/2022/day/2

# === PART 1 ===
def eval_round(move1, move2) -> int:
    move1_encoded = ord(move1)-64
    move2_encoded = ord(move2)-87

    if move1_encoded == move2_encoded:
        return move2_encoded + 3

    if (move1_encoded, move2_encoded) == (1, 2) or \
       (move1_encoded, move2_encoded) == (2, 3) or \
       (move1_encoded, move2_encoded) == (3, 1):
       return move2_encoded + 6
    
    return move2_encoded

with open("../data/2.txt") as file:
    lines = file.readlines()

print("Part 1")
print(sum([eval_round(*line.split()) for line in lines]))


# === PART 2 ===
print("Part 2")
def roll(arr, k):
    # roll/rotate list
    # roll([1, 2, 3], 1) -> [2, 3, 1]
    for _ in range(k):
        arr = arr[1:] + [arr[0]]
    return arr

def eval_round_2(move1, result) -> int:
    cycle = ord(result)-88
    move2 = roll(["Z", "X", "Y"], cycle)[ord(move1)-65]
    return eval_round(move1, move2)

print(sum([eval_round_2(*line.split()) for line in lines]))
