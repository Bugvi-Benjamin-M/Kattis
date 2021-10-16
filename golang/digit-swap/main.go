package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {

	reader := bufio.NewReader(os.Stdin)
	text, _ := reader.ReadString('\n')

	first_digit := text[0]
	second_digit := text[1]

	result := string(second_digit) + string(first_digit)
	fmt.Println(result)
}
