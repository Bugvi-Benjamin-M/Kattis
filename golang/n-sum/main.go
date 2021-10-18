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

	sum := 0
	for i := 0; i < n; i++ {
		curr, _ := strconv.Atoi(splits[i])
		sum += curr
	}

	fmt.Println(sum)

}
