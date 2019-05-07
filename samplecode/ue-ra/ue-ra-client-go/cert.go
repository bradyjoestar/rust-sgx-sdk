package main

import (
	"bytes"
	"crypto/x509"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"log"
)

func verify_mra_cert(rawCerts [][]byte, verifiedChains [][]*x509.Certificate) error {
	printCert(rawCerts[0])

	// get the pubkey and payload from raw data
	pub_k, payload := unmarshalCert(rawCerts[0])

	// Load Intel CA, Verify Cert and Signature
	attn_report_raw, err := verifyCert(payload)
	if err != nil {
		log.Fatalln(err)
		return err
	}

	// Verify attestation report
	err = verifyAttReport(attn_report_raw, pub_k)
	if err != nil {
		log.Fatalln(err)
		return err
	}

	return nil
}

func unmarshalCert(rawbyte []byte) ([]byte, []byte) {
	// Search for Public Key prime256v1 OID
	prime256v1_oid := []byte{0x06, 0x08, 0x2A, 0x86, 0x48, 0xCE, 0x3D, 0x03, 0x01, 0x07}
	offset := uint(bytes.Index(rawbyte, prime256v1_oid))
	offset += 11 // 10 + TAG (0x03)

	// Obtain Public Key length
	length := uint(rawbyte[offset])
	if length > 0x80 {
		length = uint(rawbyte[offset+1])*uint(0x100) + uint(rawbyte[offset+2])
		offset += 2
	}

	// Obtain Public Key
	offset += 1
	pub_k := rawbyte[offset+2 : offset+length] // skip "00 04"

	// Search for Netscape Comment OID
	ns_cmt_oid := []byte{0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x86, 0xF8, 0x42, 0x01, 0x0D}
	offset = uint(bytes.Index(rawbyte, ns_cmt_oid))
	offset += 12 // 11 + TAG (0x04)

	// Obtain Netscape Comment length
	length = uint(rawbyte[offset])
	if length > 0x80 {
		length = uint(rawbyte[offset+1])*uint(0x100) + uint(rawbyte[offset+2])
		offset += 2
	}

	// Obtain Netscape Comment
	offset += 1
	payload := rawbyte[offset : offset+length]
	return pub_k, payload
}

func verifyCert(payload []byte) ([]byte, error) {
	// Extract each field
	pl_split := bytes.Split(payload, []byte{0x7C})
	attn_report_raw := pl_split[0]
	sig_raw := pl_split[1]

	var sig, sig_cert_dec []byte
	sig, err := base64.StdEncoding.DecodeString(string(sig_raw))
	if err != nil {
		log.Fatalln(err)
		return nil, err
	}

	sig_cert_raw := pl_split[2]
	sig_cert_dec, err = base64.StdEncoding.DecodeString(string(sig_cert_raw))
	if err != nil {
		log.Fatalln(err)
		return nil, err
	}

	certnew, err := x509.ParseCertificate(sig_cert_dec)
	if err != nil {
		log.Fatalln(err)
		return nil, err
	}

	roots := x509.NewCertPool()
	cacert, err := readFile("./../../cert/AttestationReportSigningCACert.pem")
	if err != nil {
		log.Fatalln(err)
		return nil, err
	}
	ok := roots.AppendCertsFromPEM([]byte(cacert))
	if !ok {
		panic("failed to parse root certificate")
	}

	opts := x509.VerifyOptions{
		Roots: roots,
	}

	if _, err := certnew.Verify(opts); err != nil {
		log.Fatalln(err)
		return nil, err
	} else {
		fmt.Println("Cert is good")
	}

	// Verify the signature against the signing cert
	err = certnew.CheckSignature(certnew.SignatureAlgorithm, attn_report_raw, sig)
	if err != nil {
		log.Fatalln(err)
		return nil, err
	} else {
		fmt.Println("Signature good")
	}
	return attn_report_raw, nil
}

func verifyAttReport(attn_report_raw []byte, pub_k []byte) error {
	var qr QuoteReport
	fmt.Println(string(attn_report_raw))
	json.Unmarshal(attn_report_raw, &qr)

	qb, err := base64.StdEncoding.DecodeString(qr.IsvEnclaveQuoteBody)
	if err != nil {
		return err
	}
	var quoteString, quoteStringTwo string
	for _, b := range qb {
		quoteString += fmt.Sprint(int(b),", ")
		quoteStringTwo += fmt.Sprintf("%02x", int(b))
	}

	fmt.Println("Quote = [" + quoteString[:len(quoteString)-2] + "]")

	fmt.Println(quoteStringTwo[224:288])
	fmt.Println(quoteStringTwo[352:416])
	fmt.Println(quoteStringTwo[736:864])

	return nil
}
