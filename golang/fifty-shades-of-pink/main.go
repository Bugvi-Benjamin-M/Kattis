package main

import (
	"fmt"
	"strings"
)

func main() {

	n := parse_int()
	counter := 0

	for i := 0; i < n; i++ {
		input := parse_input()
		input_trim := strings.ToLower(input)
		if strings.Contains(input_trim, "pink") || strings.Contains(input_trim, "rose") {
			counter += 1
		}

	}

	if counter == 0 {
		fmt.Println("I must watch Star Wars with my daughter")
	} else {
		fmt.Println(counter)
	}
}
