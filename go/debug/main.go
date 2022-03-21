package main

import (
	"time"

	"gitee.com/pt_mall/utils/ptlog"
	"gitee.com/pt_mall/utils/rpc"
)

const (
	appName = "debug_2.0"
)

var (
	rpcClient rpc.Client

	infoZapLogger, errorZapLogger = ptlog.EcszapInfoAndErrorLogger(appName)
)

type Debug struct{}

func (*Debug) Demo(args struct{ Name string }, reply *rpc.CommonReply) error {
	infoZapLogger.Info("Demo start!")

	time.Sleep(10 * time.Second)

	infoZapLogger.Info("Demo end!")

	reply.Data = map[string]string{
		"Name": "Haha, " + args.Name,
	}

	return nil
}

func main() {
	rpcTool := rpc.New(appName, infoZapLogger)
	rpcClient = rpcTool.GetClient()
	rpcTool.StartServer(&Debug{})
}
