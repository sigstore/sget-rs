package main

import (
	"flag"
	"fmt"
	"os"
)

func main() {

	// noExec := flag.Bool("noexec", false, "Do not execute script, only validate")
	// outFile := flag.String("download", "", "download script to local storage")
	flag.Parse()

	if len(flag.Args()) > 1 {
		fmt.Println("only provide a single argument to sget (the registry location")
		os.Exit(1)

	}
}
