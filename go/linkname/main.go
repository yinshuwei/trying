package main

import (
	"fmt"
	_ "unsafe"

	_ "github.com/yinshuwei/trying/go/linkname/pack"
)

//go:linkname data github.com/yinshuwei/trying/go/linkname/pack.data
var data string

func main() {
	fmt.Println(data)
}
