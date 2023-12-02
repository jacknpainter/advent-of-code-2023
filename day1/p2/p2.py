#/usr/bin/python3

import regex as re

def sum(nums):
    sum = 0
    for num in nums:
        sum += num

    return sum

def parse_lines(lines):
    nums = []
    for line in lines:
        nums.append(parse_line(line))
    return sum(nums)
    

def parse_line(line):
    matches = re.findall(r'(?=(\d|one|two|three|four|five|six|seven|eight|nine))', line)
    
    firstDigit = ""
    if matches[0] == "one":
        firstDigit = "1"
    elif matches[0] == "two":
        firstDigit = "2"
    elif matches[0] == "three":
        firstDigit = "3"
    elif matches[0] == "four":
        firstDigit = "4"
    elif matches[0] == "five":
        firstDigit = "5"
    elif matches[0] == "six":
        firstDigit = "6"
    elif matches[0] == "seven":
        firstDigit = "7"
    elif matches[0] == "eight":
        firstDigit = "8"
    elif matches[0] == "nine":
        firstDigit = "9"
    else:
        firstDigit = matches[0]
    
    lastDigit = ""
    if matches[len(matches)-1] == "one":
        lastDigit = "1"
    elif matches[len(matches)-1] == "two":
        lastDigit = "2"
    elif matches[len(matches)-1] == "three":
        lastDigit = "3"
    elif matches[len(matches)-1] == "four":
        lastDigit = "4"
    elif matches[len(matches)-1] == "five":
        lastDigit = "5"
    elif matches[len(matches)-1] == "six":
        lastDigit = "6"
    elif matches[len(matches)-1] == "seven":
        lastDigit = "7"
    elif matches[len(matches)-1] == "eight":
        lastDigit = "8"
    elif matches[len(matches)-1] == "nine":
        lastDigit = "9"
    else:
        lastDigit = matches[len(matches)-1]

    num = firstDigit + lastDigit

    return int(num)

def main():
    file1 = open('text.txt', 'r')
    lines = file1.readlines()
    print(parse_lines(lines))

if __name__ == "__main__":
    main()