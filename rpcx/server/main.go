package main

import (
	"context"
	"flag"
	"log"
	"time"

	metrics "github.com/rcrowley/go-metrics"
	"github.com/smallnest/rpcx/server"
	"github.com/smallnest/rpcx/serverplugin"
)

type Args struct {
	A int
	B int
}

type Reply struct {
	C int
}

type Arith int

func (t *Arith) Mul(ctx context.Context, args *Args, reply *Reply) error {
	reply.C = args.A * args.B
	log.Println(args.A, args.B)
	return nil
}

func main() {
	flag.Parse()

	s := server.NewServer()
	addRegistryPlugin(s)

	s.RegisterName("Arith", new(Arith), "")
	s.Serve("tcp", "localhost:8971")
}

func addRegistryPlugin(s *server.Server) {
	r := &serverplugin.EtcdRegisterPlugin{
		ServiceAddress: "tcp@localhost:8971",
		EtcdServers:    []string{"127.0.0.1:2379"},
		BasePath:       "/rpcx",
		Metrics:        metrics.NewRegistry(),
		UpdateInterval: time.Second,
	}
	err := r.Start()
	if err != nil {
		log.Println(err)
	}
	s.Plugins.Add(r)
}
