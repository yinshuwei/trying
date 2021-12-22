// package main

// func main() {
// 	// open a new index
// 	mapping := bleve.NewIndexMapping()
// 	index, err := bleve.New("example1.bleve", mapping)
// 	if err != nil {
// 		fmt.Println(err)
// 		return
// 	}

// 	data := struct {
// 		Name string
// 	}{
// 		Name: "text",
// 	}

// 	// index some data
// 	index.Index("id", data)

// 	// search for some text
// 	query := bleve.NewMatchQuery("text")
// 	search := bleve.NewSearchRequest(query)
// 	searchResults, err := index.Search(search)
// 	if err != nil {
// 		fmt.Println(err)
// 		return
// 	}
// 	fmt.Println(searchResults)
// }

package main

import (
	"log"

	"github.com/blevesearch/bleve/v2"
)

func main() {
	{
		message := struct {
			Id   string
			From string
			Body string
		}{
			Id:   "example",
			From: "marty.schoch@gmail.com",
			Body: "bleve indexing is easy，mary had a little lamb",
		}

		mapping := bleve.NewIndexMapping()
		index, err := bleve.New("example.bleve", mapping)
		if err != nil {
			panic(err)
		}
		index.Index(message.Id, message)
	}

	{
		index, _ := bleve.Open("example.bleve")
		query := bleve.NewQueryStringQuery("little")
		searchRequest := bleve.NewSearchRequest(query)
		searchResult, _ := index.Search(searchRequest)

		log.Println(searchResult)
	}
}
