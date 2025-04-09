package main

import(
	"fmt"
	"net/http"
	//"github.com/gin-gonic/gin"
)

func handler(response http.ResponseWriter, r *http.Request){
	fmt.Fprint(response, "I hate my life")
}

func main(){
	http.HandleFunc("/",handler)
	http.ListenAndServe(":8080",nil)
}