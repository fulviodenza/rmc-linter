package main

import "fmt"

const TIPS = "Click on any AST node with a '+' to expand it hovering over a node highlights the corresponding location in the source code shift click on an AST node to expand the whole subtree"

func PrintTips() {
	fmt.Print(TIPS)
	var a []string = []string{"Ciao", "mondo", "come", "va?"}
	for i := range a {
		fmt.Println(a[i])
	}
}
