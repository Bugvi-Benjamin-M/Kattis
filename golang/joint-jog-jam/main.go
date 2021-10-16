package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func main() {

	reader := bufio.NewReader(os.Stdin)
	text, _ := reader.ReadString('\n')

	splits := strings.Fields(text)

	kari_start_x, _ := strconv.Atoi(splits[0])
	kari_start_y, _ := strconv.Atoi(splits[1])
	ola_start_x, _ := strconv.Atoi(splits[2])
	ola_start_y, _ := strconv.Atoi(splits[3])

	kari_end_x, _ := strconv.Atoi(splits[4])
	kari_end_y, _ := strconv.Atoi(splits[5])
	ola_end_x, _ := strconv.Atoi(splits[6])
	ola_end_y, _ := strconv.Atoi(splits[7])

	start_dist := math.Sqrt(math.Pow(float64(kari_start_x-ola_start_x), 2) + math.Pow(float64(kari_start_y-ola_start_y), 2))
	end_dist := math.Sqrt(math.Pow(float64(kari_end_x-ola_end_x), 2) + math.Pow(float64(kari_end_y-ola_end_y), 2))

	fmt.Println(math.Max(start_dist, end_dist))

}
