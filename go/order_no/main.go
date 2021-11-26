package main

import (
	"log"
	"strconv"
	"strings"
	"sync"
	"time"
)

var (
	orderNoSortInt int64 = -1
	orderNoSortMux sync.Mutex
	base32IntChars = []string{"0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H", "J", "K", "M", "N", "P", "Q", "R", "S", "T", "V", "W", "X", "Y", "Z"}
)

func base32Int(v int64) string {
	if v == 0 {
		return "0"
	}
	var chars []string
	for v > 0 {
		c := v % 32
		v = v / 32
		chars = append(chars, base32IntChars[c])
	}
	for i, j := 0, len(chars)-1; i < j; i, j = i+1, j-1 {
		chars[i], chars[j] = chars[j], chars[i]
	}
	return strings.Join(chars, "")
}

func _genOrderNoSortInt() int64 {
	orderNoSortMux.Lock()
	defer orderNoSortMux.Unlock()
	orderNoSortInt++
	if orderNoSortInt >= 32 {
		orderNoSortInt = 0
	}
	return orderNoSortInt
}

func _genOrderNoSortIntChar() string {
	return base32Int(_genOrderNoSortInt())
}

func genOrderNo(userId int64) string {
	u := base32Int(userId)
	for len(u) < 7 {
		u = "0" + u
	}

	ts, _ := strconv.ParseInt(time.Now().UTC().Format("060102150405"), 10, 64)

	return base32Int(ts) + u + _genOrderNoSortIntChar()
}

func main() {
	for i := 0; i < 100; i++ {
		log.Println(genOrderNo(int64(i)))
	}
}
