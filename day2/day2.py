import re
day2 = open("day2.txt").readlines()
day2 = [line.strip() for line in day2]

answer1 = 0

redmax = 12
greenmax = 13
bluemax = 14

for i in range(len(day2)):
    trueflag = True
    index = day2[i].find(":") + 1
    newline = day2[i][index:]
    diceshown = newline.split("; ")
    for dice in diceshown:
        dice = dice.strip()
        colours = dice.split(", ")
        for color in colours:
            number, colour = color.split(" ")
            number = int(number)
            # print(number, colour)
            if colour == "red" and number > redmax:
                trueflag = False
                break
            if colour == "green" and number > greenmax:
                trueflag = False
                break           
            if colour == "blue" and number > bluemax:
                trueflag = False
                break
    if trueflag:
        answer1 += (i + 1)

print(answer1)

part2answer = 0

for i in range(len(day2)):
    trueflag = True
    index = day2[i].find(":") + 1
    newline = day2[i][index:]
    diceshown = newline.split("; ")
    redmax = 0
    greenmax = 0
    bluemax = 0
    for dice in diceshown:
        dice = dice.strip()
        colours = dice.split(", ")
        for color in colours:
            number, colour = color.split(" ")
            number = int(number)
            if colour == "red" and number > redmax:
                redmax = number
            if colour == "green" and number > greenmax:
                greenmax = number
            if colour == "blue" and number > bluemax:
                bluemax = number
    part2answer += (redmax * greenmax * bluemax)

print(part2answer)