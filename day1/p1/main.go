package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	pwd, _ := os.Getwd()

	// read file
	file, err := os.Open(pwd + "/text.txt")
	if err != nil {
		panic(err)
	}
	defer file.Close()

	lines := []string{}
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	nums := []int{}
	// regex numbers
	re := regexp.MustCompile(`(\d|one|two|three|four|five|six|seven|eight|nine)`)

	for _, line := range lines {
		grps := re.FindStringSubmatch(line)
		fmt.Println(grps)

		groups := re.FindAllString(line, -1)

		var firstDigit string
		switch groups[0] {
		case "one":
			firstDigit = "1"
		case "two":
			firstDigit = "2"
		case "three":
			firstDigit = "3"
		case "four":
			firstDigit = "4"
		case "five":
			firstDigit = "5"
		case "six":
			firstDigit = "6"
		case "seven":
			firstDigit = "7"
		case "eight":
			firstDigit = "8"
		case "nine":
			firstDigit = "9"
		default:
			firstDigit = groups[0]
		}

		var lastDigit string
		switch groups[len(groups)-1] {
		case "one":
			lastDigit = "1"
		case "two":
			lastDigit = "2"
		case "three":
			lastDigit = "3"
		case "four":
			lastDigit = "4"
		case "five":
			lastDigit = "5"
		case "six":
			lastDigit = "6"
		case "seven":
			lastDigit = "7"
		case "eight":
			lastDigit = "8"
		case "nine":
			lastDigit = "9"
		default:
			lastDigit = groups[len(groups)-1]
		}

		num := firstDigit + lastDigit

		fmt.Printf("line: %s\nfirstDigit: %s\nlastDigit: %s\nnum: %s\n\n", line, groups[0], groups[len(groups)-1], num)

		iNum, err := strconv.Atoi(num)
		if err != nil {
			panic(err)
		}

		nums = append(nums, iNum)
	}

	// sum numbers
	fmt.Println(sum(nums))
}

func sum(nums []int) int {
	sum := 0
	for _, i := range nums {
		sum += i
	}

	return sum
}
