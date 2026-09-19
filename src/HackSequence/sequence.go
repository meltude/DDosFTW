package HackSequence

import (
	"fmt"
	"time"
)

type Step struct {
	Message            string
	MinDelay, MaxDelay time.Duration
}

func printSteps(steps []Step) {
	for _, step := range steps {
		fmt.Println(step.Message)
		time.Sleep(RandDelay(step))
	}
}

func FakeHackSequence(target string) {
	printSteps([]Step{
		{fmt.Sprintf("resolving domain %s...", target), 600 * time.Millisecond, 1000 * time.Millisecond},
		{fmt.Sprintf("target IP located: %s", RandomIP()), 400 * time.Millisecond, 800 * time.Millisecond},
		{fmt.Sprintf("scanning open ports on %s...", target), 1200 * time.Millisecond, 2000 * time.Millisecond},
		{fmt.Sprintf("port %d found open", RandomPort()), 300 * time.Millisecond, 700 * time.Millisecond},
		{"identifying server software...", 800 * time.Millisecond, 1300 * time.Millisecond},
		{"server fingerprint: nginx/1.18.0", 500 * time.Millisecond, 900 * time.Millisecond},
		{"spoofing MAC address...", 600 * time.Millisecond, 1200 * time.Millisecond},
		{fmt.Sprintf("new MAC: %s", RandomMAC()), 300 * time.Millisecond, 700 * time.Millisecond},
		{"bypassing firewall...", 1500 * time.Millisecond, 2500 * time.Millisecond},
		{"injecting payload...", 1000 * time.Millisecond, 2000 * time.Millisecond},
		{"root access obtained", 500 * time.Millisecond, 900 * time.Millisecond},
		{fmt.Sprintf("accessing root directory of %s...", target), 600 * time.Millisecond, 1200 * time.Millisecond},
		{"extracting data...", 1400 * time.Millisecond, 2200 * time.Millisecond},
		{"disconnecting...", 300 * time.Millisecond, 700 * time.Millisecond},
	})
}

func FakeAccountDemo(service, account string) {
	printSteps([]Step{
		{fmt.Sprintf("loading fictional profile for %s...", service), 600 * time.Millisecond, 1000 * time.Millisecond},
		{fmt.Sprintf("selected account: %s", account), 400 * time.Millisecond, 800 * time.Millisecond},
		{"trying to figure out session token", 800 * time.Millisecond, 1300 * time.Millisecond},
		{fmt.Sprintf("session token ID: %s", RandomHex(16)), 500 * time.Millisecond, 900 * time.Millisecond},
		{"hacking sequence finished without any problems!", 400 * time.Millisecond, 800 * time.Millisecond},
	})
}
