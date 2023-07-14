package main

import (
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
)

func loadData() string {
	data, err := os.ReadFile("data.txt")
	if err != nil {
		log.Fatal(err)
	}
	return string(data)
}

func getElvesCategories() []int {
	data := loadData()
	lines := strings.Split(data, "\n")
	elvesCategories := make([]int, 0)
	currentTotal := 0
	for _, line := range lines {
		if line == "" {
			elvesCategories = append(elvesCategories, currentTotal)
			currentTotal = 0
		} else {
			value, err := strconv.Atoi(line)
			if err != nil {
				log.Fatal(err)
			}
			currentTotal += value
		}
	}
	return elvesCategories
}

func part1() {
	numbers := getElvesCategories()
	largest := 0
	for _, number := range numbers {
		if number > largest {
			largest = number
		}
	}
	log.Println("Largest:", largest)
}

func part2() {
	numbers := getElvesCategories()
	sort.Sort(sort.Reverse(sort.IntSlice(numbers)))
	topThreeTotal := numbers[0] + numbers[1] + numbers[2]
	log.Println("Top three total:", topThreeTotal)
}

func main() {
	part1()
	part2()
}
