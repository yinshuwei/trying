package main

import "log"

// MyString MyString
type MyString string

func (s MyString) print() {
	log.Println(s)
}

func main() {
	MyString("Haha").print()
}
