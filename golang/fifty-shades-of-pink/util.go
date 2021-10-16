package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"
)

var reader = bufio.NewReader(os.Stdin)

func parse_int() int {
	s, err := reader.ReadString('\n')
	if err != nil {
		panic("Failed to parse from stdin!")
	}
	s = strings.TrimSuffix(s, "\n")
	n, err := strconv.Atoi(s)
	if err != nil {
		panic("Failed to parse integer!")
	}
	return n
}

func parse_input() string {
	s, err := reader.ReadString('\n')
	if err != nil {
		panic("Failed to parse from stdin!")
	}
	s = strings.TrimSuffix(s, "\n")
	return s
}
