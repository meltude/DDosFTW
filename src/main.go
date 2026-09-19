package main

import (
	"Wallfox/src/HackSequence"
	logo "Wallfox/src/Logo"
	"bufio"
	"fmt"
	"os"
	"strings"
)

func readInput(r *bufio.Reader, prompt string) (string, bool) {
	for {
		fmt.Print(prompt)
		value, _ := r.ReadString('\n')
		value = strings.TrimSpace(value)
		if strings.EqualFold(value, "q") || strings.EqualFold(value, "quit") {
			return "", false
		}
		if value != "" {
			return value, true
		}
		fmt.Println("Err: This field cannot be empty.")
	}
}

func main() {
	logo.PrintLogo()
	r := bufio.NewReader(os.Stdin)
	for {
		fmt.Print("\nChoose mode: [1] domain hack  [2] account hack [q] quit: ")
		mode, _ := r.ReadString('\n')
		mode = strings.TrimSpace(strings.ToLower(mode))
		if mode == "q" || mode == "quit" {
			fmt.Println("exit")
			return
		}

		if mode == "2" {
			service, ok := readInput(r, "Service name (or 'q' to cancel): ")
			if !ok {
				continue
			}
			account, ok := readInput(r, "Username/email for the hacking account (or 'q' to cancel): ")
			if !ok {
				continue
			}
			HackSequence.FakeAccountDemo(service, account)
		} else {
			target, ok := readInput(r, "Write domain here (or 'q' to quit): ")
			if !ok {
				fmt.Println("exit")
				return
			}
			HackSequence.FakeHackSequence(target)
			fmt.Printf("\n%s has been successfully hacked\n", target)
		}

		fmt.Print("\nYou have another target? (press Enter to continue, or 'q' to quit): ")
		again, _ := r.ReadString('\n')
		if strings.EqualFold(strings.TrimSpace(again), "q") || strings.EqualFold(strings.TrimSpace(again), "quit") {
			fmt.Println("exit")
			return
		}
	}
}
