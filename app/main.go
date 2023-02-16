package main

import (
	handler "app/gen"
)

func init() {
	a := HttpImpl{}
	handler.SetHandler(a)
}

type HttpImpl struct {
}

func (i HttpImpl) HandleHttp(req handler.HandlerRequest) handler.Result[handler.HandlerResponse, handler.HandlerHttpError] {
	for _, header := range req.Headers {
		println(header.F0)
		println(header.F1)
	}
	for _, arg := range req.Params {
		println(arg.F0)
		println(arg.F1)
	}
	response := handler.HandlerResponse{}
	response.Status = 200
	response.Body = handler.Some([]byte("hello world!"))

	var res handler.Result[handler.HandlerResponse, handler.HandlerHttpError]
	res.Set(response)
	return res
}

//go:generate wit-bindgen tiny-go ../wit/world.wit --out-dir=gen
func main() {}
