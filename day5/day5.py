day5 = open("day5test.txt").readlines()

seeds = "79 14 55 13"
# seeds = "1044452533 40389941 3710737290 407166728 1552449232 639689359 3327654041 26912583 3440484265 219136668 1126550158 296212400 2332393052 229950158 200575068 532702401 4163696272 44707860 3067657312 45353528"

seeds = seeds.split(' ')

for i in range(len(seeds)):
    seeds[i] = int(seeds[i])

overallmap = []

current_map = []

index = 1

while index < len(day5):
    if day5[index] == '\n':
        index += 2
        overallmap.append(current_map)
        current_map = []
    else:
        line = day5[index].strip()
        dst, src, rng = line.split(' ')
        coord = (int(dst), int(src), int(rng))
        current_map.append(coord)
        index += 1
overallmap.append(current_map)
# print(overallmap)
destination_array = []

for seed in seeds:
    final = seed
    for i in range(len(overallmap)):
        for j in range(len(overallmap[i])):
            dst, src, rng = overallmap[i][j]
            # print(dst, src, rng)
            if src <= final <= (src + rng):
                final += dst - src
                break
        # print("interim:", final)
    # print("final:", final)
    destination_array.append(final)

print(min(destination_array))

seedset = set()

index = 2

overlaps_array = [(seeds[0], seeds[0]+ seeds[1])]

while index < len(seeds):
    sent = False
    for i in range(len(overlaps_array)):
        lower = seeds[index]
        upper = seeds[index] + seeds[index + 1]
        if (lower < overlaps_array[i][0] and upper > overlaps_array[i][0]) or (lower < overlaps_array[i][1] and upper > overlaps_array[i][1]) or (lower > overlaps_array[i][0] and upper < overlaps_array[i]):
            newlower = min(lower, overlaps_array[i][0])
            newupper = max(upper, overlaps_array[i][1])
            overlaps_array[i] = (newlower, newupper)
            sent = True
            break
    if sent == False:
        overlaps_array.append((seeds[index], seeds[index] + seeds[index + 1]))
    index += 2

# print(len(overlaps_array))

# print(overlaps_array[0][0] - overlaps_array[1][1])
# print(overlaps_array[0][1], overlaps_array[1][0])
# if overlaps_array[0][1] > overlaps_array[1][0]:
#     print("yes")
print(overlaps_array)
