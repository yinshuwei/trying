package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"strconv"
	"strings"
	"sync"

	"encoding/csv"
	"encoding/json"
)

const fileName = "customer_address_201912151523.csv"

func main() {
	log.SetFlags(log.Lshortfile | log.Ldate | log.Ltime)

	f, err := os.Open(fileName)
	if err != nil {
		log.Println(err)
	}
	defer f.Close()

	r := csv.NewReader(f)
	rs, err := r.ReadAll()
	if err != nil {
		log.Println(err)
	}

	var newRs = [][]string{}
	newRs = append(newRs, []string{"id", "address", "street", "location_x", "location_y", "sys_mark"})
	wg := sync.WaitGroup{}
	for range rs {
		wg.Add(1)
	}
	var jc = make(chan int, 8)
	for index, data := range rs {
		if index == 0 {
			wg.Done()
			continue
		}
		println(index)
		id := strings.ReplaceAll(data[0], ".0", "")
		address := strings.ReplaceAll(data[1], "\n", "")
		street := strings.ReplaceAll(data[2], "\n", "")
		locationX := strings.ReplaceAll(data[3], "\n", "")
		locationY := strings.ReplaceAll(data[4], "\n", "")
		// sysMark := strings.ReplaceAll(data[5], "\n", "")
		sysMark := ""

		newData := []string{id, address, street, locationX, locationY, sysMark}
		newRs = append(newRs, newData)
		if sysMark == "bd" || sysMark == "bdf" {
			wg.Done()
			continue
		}

		jc <- 1
		go func(d []string) {
			defer func() {
				<-jc
				wg.Done()
			}()

			address := "上海"
			if d[1] != "nan" {
				address = address + d[1]
			}
			if d[2] != "nan" {
				address = address + d[2]
			}

			url := fmt.Sprintf("http://api.map.baidu.com/geocoding/v3/?address=%s&output=json&ak=f1IIBRbRvhBt0L7vgeYNw3eUkNUhv6yL", address)
			resp, err := http.Get(url)
			if err != nil {
				log.Println(err)
				d[5] = "bdf"
				return
			}
			defer resp.Body.Close()
			b, err := ioutil.ReadAll(resp.Body)
			if err != nil {
				log.Println(err)
				d[5] = "bdf"
				return
			}
			var dd struct {
				Result struct {
					Location struct {
						Lng float64 `json:"lng"`
						Lat float64 `json:"lat"`
					} `json:"location"`
				} `json:"result"`
			}
			log.Println(string(b))
			err = json.Unmarshal(b, &dd)
			if err != nil {
				log.Println(err)
				d[5] = "bdf"
				return
			}
			d[3] = strconv.FormatFloat(dd.Result.Location.Lng, 'f', 16, 64)
			d[4] = strconv.FormatFloat(dd.Result.Location.Lat, 'f', 16, 64)
			d[5] = "bd"
		}(newData)
	}

	wg.Wait()

	file, err := os.Create("result_" + fileName)
	if err != nil {
		log.Println(err)
		return
	}
	defer file.Close()

	writer := csv.NewWriter(file)
	defer writer.Flush()
	writer.WriteAll(newRs)
}
