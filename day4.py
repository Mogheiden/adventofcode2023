day4 = open("day4.txt").readlines()
day4 = [line.strip() for line in day4]

part1answer = 0
array = [1] * len(day4)

for i in range(len(day4)):
    linepoints = 0
    numwins = 0
    index = day4[i].find(":") + 1
    newline = day4[i][index:]

    winning, having = newline.split("|")
    winning = winning.strip().split(' ')
    having = having.strip().split(' ')
    for number in having:
        if number != '' and number in winning:
            if linepoints == 0:
                linepoints = 1
            else:
                linepoints *= 2
            numwins += 1
    part1answer += linepoints
    for j in range(numwins):
        array[i + j + 1] += array[i]

print(part1answer)
print(sum(array))

