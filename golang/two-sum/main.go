package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {

	reader := bufio.NewReader(os.Stdin)
	text, _ := reader.ReadString('\n')

	splits := strings.Fields(text)

	num1, _ := strconv.Atoi(splits[0])
	num2, _ := strconv.Atoi(splits[1])
	sum := num1 + num2

	fmt.Println(sum)
}
