package main

import (
	"crypto/tls"
	"io/ioutil"
	"log"
	"os"
)

const SERVERADDR = "localhost:3443"

func main() {
	log.SetFlags(log.Lshortfile)
	println("Starting ue-ra-client-go")

	certPem, keyPem := loadCert()
	pem := []byte(certPem + keyPem)
	cert, err := tls.X509KeyPair(pem, pem)
	if err != nil {
		log.Fatalln(err)
	}

	println("Connecting to ", SERVERADDR)

	conn, err := tls.Dial("tcp", SERVERADDR, make_config(cert))
	if err != nil {
		log.Fatalln(err)
	}
	defer conn.Close()

	n, err := conn.Write([]byte("hello ue-ra go client"))
	if err != nil {
		log.Fatalln(err)
	}

	buf := make([]byte, 100)
	n, err = conn.Read(buf)
	if err != nil {
		log.Fatalln(err)
	}

	println("server replied: ", string(buf[:n]))
}

func loadCert() (string, string) {
	certPem, err := readFile("./../../cert/client.crt")
	if err != nil {
		log.Fatalln(err)
	}

	keyPEM, err := readFile("./../../cert/client.pkcs8")
	if err != nil {
		log.Fatalln(err)
	}
	return certPem, keyPEM
}

func readFile(filePth string) (string, error) {
	f, err := os.Open(filePth)
	if err != nil {
		return "", err
	}
	content, err := ioutil.ReadAll(f)
	if err != nil {
		return "", err
	}
	return string(content), nil
}

func make_config(cert tls.Certificate) *tls.Config {
	conf := &tls.Config{
		InsecureSkipVerify: true,
	}
	conf.Certificates = []tls.Certificate{cert}
	conf.VerifyPeerCertificate = verify_mra_cert
	return conf
}
