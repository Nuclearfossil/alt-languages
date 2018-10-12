package main

import (
	"fmt"
	"net/http"
)

func DebugResponse(r *http.Request){
	fmt.Println("Incoming Connection")
	fmt.Println(r.Method)
	fmt.Println(r.Header)
	fmt.Println(r.Trailer)
	fmt.Println(r.Body)
}
