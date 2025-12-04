import collections
from typing import DefaultDict


example = """987654321111111
811111111111119
234234234234278
818181911112111"""

batteries_example: list[list[int]] = []
for line in example.split("\n"):
    batteries_example.append([int(e) for e in line])


batteries : list[list[int]] = []
with open("input.txt", "r") as f:
    for line in f.readlines():
        batteries.append([int(e) for e in line.strip()])


def part1(batteries : list[list[int]]):
    sum = 0
    for b in batteries:
        first= b[0]
        volt=0
        for e in b[1:]:
            volt= max(volt, first*10 +e)
            first =max(first,e)
        sum+=volt
    return sum
        # print(volt, "".join(map(str, b)))

print ("part1 example:", part1(batteries_example))
print ("part1:", part1(batteries))


def recursive_part2(indices : DefaultDict[int, list[int]], batteries : list[int], batteries_left : int) -> int:
    if batteries_left == 0:
        return 0
    # assert batteries_left > 0

    voltage = 0

    last_idx = 0
    for i in range(9, 0, -1):
        if indices[i] != []:
            idx = indices[i][0]
            # print(f"len: {len(batteries)} - {idx = } >= {batteries_left = }")
            # print(f"{len(batteries) - idx >= batteries_left}")
            if len(batteries) - idx >= batteries_left:
                voltage = i

                last_idx = idx

                # print(f"{ last_idx = }")
                # print(indices.values())
                for l in indices.values():
                    # print(l)
                    
                    idx_to_remove = 0
                    for (idx_to_remove, idx_of_battery) in enumerate(l):
                        # print(f"{ idx_to_remove = } >= { idx_of_battery = }")

                        if idx_of_battery > last_idx:
                            # print(f"deleting {l}[:{idx_to_remove}]")
                            del l[:idx_to_remove]
                            break
                    else:
                        # print(f"clearing {l}")
                        l.clear()

                 

                # print(f"Using battery at { idx = } with value {i}")
                # print(f"{indices = }")
                # print(f"{voltage =  }")

                # exit()
                next_voltage = recursive_part2(indices, batteries, batteries_left - 1)
                # print(f"{pow(10, batteries_left - 1) * voltage} + {next_voltage = }")
                return pow(10, batteries_left - 1) * voltage + next_voltage

    print(f"Algo correu mal {batteries_left}")
    print(f"{indices =}")
    # print(f"{batteries = }")
    raise ValueError("BARRACA")
    # return voltage 

def part2(batteries : list[list[int]]) -> int:
    NUM_BATTERIES = 12
    part2_result = 0

    for line in batteries:
        indices = collections.defaultdict(list)
        for (i, b) in enumerate(line):
            indices[b].append(i)
        
        # print(indices)
        # print("".join(map(str, line)))
        voltage = recursive_part2(indices, line, NUM_BATTERIES)
        # print(f"{voltage = }")

        part2_result += voltage

    return part2_result

print("part2 example:", part2(batteries_example))
print("part2:", part2(batteries))
