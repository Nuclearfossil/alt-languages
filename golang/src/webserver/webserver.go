package main

import (
	"fmt"
	"log"
	"net/http"
	"os"
	"path"
)

func main() {
	http.HandleFunc("/", MainPage)
	http.HandleFunc("/hi", HiPage)

	fmt.Println("Starting Web Service")
	log.Fatal(http.ListenAndServe(":8081", nil))
}

func MainPage(w http.ResponseWriter, r *http.Request) {
	DebugResponse(r)
	var rootPath = os.Getenv("GOPATH")
	http.ServeFile(w, r, path.Join(rootPath, "hello.html"))
}

func HiPage(w http.ResponseWriter, r *http.Request) {
	DebugResponse(r)
	fmt.Fprintf(w, "Hi")
}
