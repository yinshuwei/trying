package main

import (
	"context"
	"fmt"
	"log"
	"time"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())

	ctx = context.WithValue(ctx, interface{}("a"), "b")
	go func() {
		time.Sleep(1000 * time.Millisecond)
		cancel()
	}()

	go handle(ctx, 500*time.Millisecond)

	select {
	case <-ctx.Done():
		fmt.Println("main", ctx.Err())
	}

	time.Sleep(100 * time.Second)
}

func handle(ctx context.Context, duration time.Duration) {
	log.Println("value a:", ctx.Value("a"))
	select {
	case <-ctx.Done():
		fmt.Println("handle", ctx.Err())

	case <-time.After(duration):
		fmt.Println("process request with", duration)
	}
}
