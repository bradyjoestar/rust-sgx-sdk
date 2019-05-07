package main

import (
	"crypto/x509"
	"fmt"
	"log"
)

func verify_mra_cert(rawCerts [][]byte, verifiedChains [][]*x509.Certificate) error {
	cert, err := x509.ParseCertificate(rawCerts[0])
	if err != nil {
		log.Fatalln(err)
		return err
	}
	printCert(cert.Raw)

	return nil
}

func printCert(rawByte []byte) {
	print("--received-server cert: [Certificate(b\"")
	for _, b := range rawByte {
		if b == '\n' {
			print("\\n")
		} else if b == '\r' {
			print("\\r")
		} else if b == '\t' {
			print("\\t")
		} else if b == '\\' || b == '"' {
			print("\\",string(rune(b)))
		} else if b >= 0x20 && b < 0x7f {
			print(string(rune(b)))
		} else {
			fmt.Printf("\\x%02x", int(b))
		}
	}
	println("\")]")
}
