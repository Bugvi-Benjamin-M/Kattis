package main

import (
	"fmt"
	"strconv"
	"strings"
)

func main() {

	n := parse_int()

	input := parse_input()

	splits := strings.Fields(input)

	counter := 1
	for i := 1; i < n; i++ {

		prev, _ := strconv.Atoi(splits[i-1])
		curr, _ := strconv.Atoi(splits[i])

		if prev < curr {
			counter += 1
		}

	}

	fmt.Println(counter)

}
