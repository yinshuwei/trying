package main

import (
	"context"
	"log"

	"github.com/smallnest/rpcx/client"
)

func main() {
	d := client.NewEtcdDiscovery("/rpcx", "Arith", []string{"127.0.0.1:2379"}, nil)
	xclient := client.NewXClient("Arith", client.Failover, client.RandomSelect, d, client.DefaultOption)
	defer xclient.Close()

	for i := 0; i < 10000; i++ {
		// #3
		args := map[string]interface{}{
			"A": i,
			"B": 1,
		}

		// #4
		reply := &map[string]interface{}{}

		// #5
		err := xclient.Call(context.Background(), "Mul", args, reply)
		if err != nil {
			log.Printf("failed to call: %v", err)
		}

		log.Print(reply)
	}
}
