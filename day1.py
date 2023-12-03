import re
day1 = open("day1.txt").readlines()
day1 = [line.strip() for line in day1]

current = 0

totals_list = []

for line in day1:
    for char in line:
        if char.isdigit():
            val = (ord(char) - ord("0")) * 10
            current += val
            break
    for char in reversed(line):
        if char.isdigit():
            val = (ord(char) - ord("0"))
            current += val
            break
    
    totals_list.append(current)
    current = 0
answer1 = sum(totals_list)
print(answer1)

vals = {
    "one" : "1",
    "two" : "2",
    "three" : "3",
    "four" : "4",
    "five" : "5",
    "six" : "6",
    "seven" : "7",
    "eight" : "8",
    "nine" : "9",
    "zero" : "0",

}
total = 0

for l in open("day1.txt"):
    words = re.findall("(?=(" + "|".join(vals.keys()) + "|\d))" , l)
    
    total += int("".join([ d if d.isdigit() else vals[d] for d in [words[0] , words[-1]]]))
    
print(total)

# answer2 = sum(totals_list2)
# print(totals_list2)
# print(answer2)