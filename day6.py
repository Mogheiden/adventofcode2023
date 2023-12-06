times = [56, 71, 79, 99]
distances = [334, 1135, 1350, 2430]

# times = [7, 15, 30]
# distances = [9, 40, 200]

numberarray = []

for i in range(len(times)):
    cur_time = times[i]
    cur_dist = distances[i]
    min_time = 0
    max_time = 0

    for j in range(cur_time):
        distance = (cur_time - j) * j
        if distance > cur_dist:
            min_time = j
            break

    for k in reversed(range(cur_time)):
        distance = (cur_time - k) * k
        if distance > cur_dist:
            max_time = k
            break
    # print(max_time, min_time)
    numberarray.append(max_time - min_time + 1)

part1 = 1

for rng in numberarray:
    # print(rng)
    part1 *= rng

print(part1)

p2time = 56717999
p2dist = 334113513502430

# p2time = 71530
# p2dist = 940200

min_time = 0
max_time = 0
middle = p2time//2

while middle < p2time:
    distance = (p2time - middle) * middle
    if distance < p2dist:
        max_time = middle - 1
        break
    middle += 1

middle = p2time//2

while middle > 0:
    distance = (p2time - middle) * middle
    if distance < p2dist:
        min_time = middle + 1
        break
    middle -= 1

print(max_time - min_time + 1)