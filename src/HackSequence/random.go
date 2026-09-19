package HackSequence

import (
	"fmt"
	"math/rand"
	"time"
)

func RandDelay(s Step) time.Duration {
	delta := s.MaxDelay - s.MinDelay
	if delta <= 0 {
		return s.MinDelay
	}
	return s.MinDelay + time.Duration(rand.Int63n(int64(delta)))
}

func RandomIP() string {
	return fmt.Sprintf("%d.%d.%d.%d",
		rand.Intn(256), rand.Intn(256), rand.Intn(256), rand.Intn(256))
}

func RandomPort() int {
	return 1024 + rand.Intn(64511)
}

func RandomHex(length int) string {
	const chars = "0123456789abcdef"
	b := make([]byte, length)
	for i := range b {
		b[i] = chars[rand.Intn(len(chars))]
	}
	return string(b)
}

func RandomMAC() string {
	return fmt.Sprintf("%s:%s:%s:%s:%s:%s",
		RandomHex(2), RandomHex(2), RandomHex(2),
		RandomHex(2), RandomHex(2), RandomHex(2))
}

func RandomDumpSize() string {
	gb := 0.5 + rand.Float64()*240.0
	return fmt.Sprintf("%.1f GB", gb)
}
