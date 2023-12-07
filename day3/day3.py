import time

day3 = open("day3.txt").readlines()
day3 = [line.strip() for line in day3]

numdict = {}
part1answer = 0

for i in range(len(day3)):
    j = 0
    while j < len(day3[0]):
        # print(j)
        if day3[i][j].isdigit():
            digitstr = day3[i][j]
            offset = 1
            digitcoords = [(i,j)]
            while j + offset < len(day3[0]) and day3[i][j + offset].isdigit():
                digitstr += day3[i][j + offset]
                digitcoords.append((i, j + offset))
                offset += 1
            j += offset
            digit = int(digitstr)
            for coord in digitcoords:
                numdict[coord] = digit
        j += 1

for i in range(len(day3)):
    for j in range(len(day3[i])):
        if day3[i][j].isdigit() == False and day3[i][j] != '.':
            for k in range(i-1, i+2):
                partnumbers = set()
                if k < 0 or k >= len(day3):
                    continue
                for l in range(j-1, j+2):
                    if l < 0 or l >= len(day3[i]):
                        continue
                    if (k,l) in numdict:
                        partnumbers.add(numdict[(k,l)])
            part1answer += sum(partnumbers)

# part1answer = sum(partnumbers)
print(part1answer)

start = time.time()

part2answer = 0

for i in range(len(day3)):
    for j in range(len(day3[i])):
        if day3[i][j] == '*':
            partnumbers = set()
            for k in range(i-1, i+2):
                if k < 0 or k >= len(day3):
                    continue
                for l in range(j-1, j+2):
                    if l < 0 or l >= len(day3[i]):
                        continue
                    if (k,l) in numdict:
                        partnumbers.add(numdict[(k,l)])
            if len(partnumbers) == 2:
                answer = 1
                for part in partnumbers:
                    answer *= part
                part2answer += answer

print(part2answer)
end = time.time()

print((end - start) * 1000)
