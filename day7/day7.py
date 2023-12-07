import time
day7 = open("day7.txt").readlines()
day7 = [line.strip() for line in day7]


start = time.time()

strength_order = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2']

hands = {
"five_of_kind" : 6,
"four_of_kind" : 5,
"full_house" : 4,
"three_of_kind" : 3,
"two_pair" : 2,
"two_of_kind" : 1,
"high_card" : 0
}

def compare(tup1, tup2):
    str_index = 0
    while str_index < len(tup1[0]):
        first = strength_order.index(tup1[0][str_index])
        second = strength_order.index(tup2[0][str_index])
        if first > second:
            return -1
        elif second > first:
            return 1
        else:
            str_index += 1

def get_type(hand):
    numarray = [0] * len(strength_order)
    for char in hand:
        numarray[strength_order.index(char)] += 1
    maximum = max(numarray)
    if maximum == 5:
        return hands['five_of_kind']
    elif maximum == 4:
        return hands['four_of_kind']
    elif maximum == 3:
        if 2 in numarray:
            return hands['full_house']
        else:
            return hands['three_of_kind']
    elif maximum == 2:
        if numarray.count(2) == 2:
            return hands['two_pair']
        else:
            return hands['two_of_kind']
    elif maximum == 1:
        return hands['high_card']
    else:
        return -1

def get_type2(hand):
    numarray = [0] * len(strength_order)
    for char in hand:
        numarray[strength_order.index(char)] += 1
    maximum = max(numarray) + numarray[-1]
    if maximum >= 4:
        newarray = numarray[:-1]
        maximum = max(newarray) + numarray[-1]
    # print(numarray[-1])
    if maximum == 5:
        return hands['five_of_kind']
    elif maximum == 4:
        # print(hand)
        return hands['four_of_kind']
    elif maximum == 3:
        if numarray.count(2) == 2 or (numarray.count(3) == 1 and numarray.count(2) == 1):
            return hands['full_house']
        else:
            return hands['three_of_kind']
    elif maximum == 2:
        if numarray.count(2) == 2:
            return hands['two_pair']
        else:
            return hands['two_of_kind']
    elif maximum == 1:
        return hands['high_card']
    else:
        return -1


sorted_list = []
for i in range(7):
    sorted_list.append([])

for rnd in day7:
    hand, bet = rnd.split(' ')
    type_enum = get_type(hand)
    sorted_list[type_enum].append((hand, int(bet)))

for hand_type in sorted_list:
    hand_type.sort(compare)

part1total = 0
part1mult = 1

# print(sorted_list)

for hand_type in sorted_list:
    for hand in hand_type:
        part1total += (hand[1] * part1mult)
        part1mult += 1
        # print(part1mult, hand[0], hand[1])
        # print(part1total)
print(sorted_list[5]);
print(part1total)

part2total = 0
part2mult = 1

strength_order = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J']

sorted_list2 = []
for i in range(7):
    sorted_list2.append([])

for rnd in day7:
    hand, bet = rnd.split(' ')
    type_enum = get_type2(hand)
    sorted_list2[type_enum].append((hand, int(bet)))

for hand_type in sorted_list2:
    hand_type.sort(compare)

part2total = 0
part2mult = 1

for hand_type in sorted_list2:
    for hand in hand_type:
        # print(part2mult, hand[0], hand[1])
        # print(hand[1] * part2mult)
        part2total += (hand[1] * part2mult)
        part2mult += 1
        # print(part2total)
# print(sorted_list2[5])
print(part2total)

end = time.time()

print((end - start) * 1000)