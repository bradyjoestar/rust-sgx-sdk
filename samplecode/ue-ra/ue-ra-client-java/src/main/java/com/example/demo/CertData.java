package com.example.demo;

import java.util.List;

public class CertData {
    public List<Byte> payload;
    public byte[] pub_k;

    public CertData(List<Byte> payload, byte[] pub_k) {
        this.payload = payload;
        this.pub_k = pub_k;
    }
}
