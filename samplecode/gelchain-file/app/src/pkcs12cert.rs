use hex;
use openssl::hash::MessageDigest;

use openssl::asn1::Asn1Time;
use openssl::nid::Nid;
use openssl::pkcs12::{ParsedPkcs12, Pkcs12};
use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::{Padding, Rsa};
use openssl::x509::extension::KeyUsage;
use openssl::x509::{X509Name, X509};

pub fn create() {
    let subject_name = "ns.example.com";
    let rsa = Rsa::generate(2048).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();

    let mut name = X509Name::builder().unwrap();
    name.append_entry_by_nid(Nid::COMMONNAME, subject_name)
        .unwrap();
    let name = name.build();

    let key_usage = KeyUsage::new().digital_signature().build().unwrap();

    let mut builder = X509::builder().unwrap();
    builder.set_version(2).unwrap();
    builder
        .set_not_before(&Asn1Time::days_from_now(0).unwrap())
        .unwrap();
    builder
        .set_not_after(&Asn1Time::days_from_now(365).unwrap())
        .unwrap();
    builder.set_subject_name(&name).unwrap();
    builder.set_issuer_name(&name).unwrap();
    builder.append_extension(key_usage).unwrap();
    builder.set_pubkey(&pkey).unwrap();
    builder.sign(&pkey, MessageDigest::sha256()).unwrap();
    let cert = builder.build();

    let pkcs12_builder = Pkcs12::builder();
    let pkcs12 = pkcs12_builder
        .build("mypass", subject_name, &pkey, &cert)
        .unwrap();
    let der = pkcs12.to_der().unwrap();

    let pkcs12 = Pkcs12::from_der(&der).unwrap();
    let parsed = pkcs12.parse("mypass").unwrap();

    assert_eq!(
        &*parsed.cert.digest(MessageDigest::sha1()).unwrap(),
        &*cert.digest(MessageDigest::sha1()).unwrap()
    );
    assert!(parsed.pkey.public_eq(&pkey));
    println!("-----------------create pkcs12 success---------------------");
}

pub fn parse() {
    let der = include_bytes!("../test/identity.p12");
    let pkcs12 = Pkcs12::from_der(der).unwrap();
    let parsed: ParsedPkcs12 = pkcs12.parse("mypass").unwrap();

    assert_eq!(
        hex::encode(parsed.cert.digest(MessageDigest::sha1()).unwrap()),
        "59172d9313e84459bcff27f967e79e6e9217e584"
    );

    let chain = parsed.chain.unwrap();
    assert_eq!(chain.len(), 1);
    assert_eq!(
        hex::encode(chain[0].digest(MessageDigest::sha1()).unwrap()),
        "c0cbdf7cdd03c9773e5468e1f6d2da7d5cbb1875"
    );
    println!("-----------------parse pkcs12 success---------------------");
}

pub fn encrypt_data() {
    let der = include_bytes!("../test/identity.p12");
    let pkcs12 = Pkcs12::from_der(der).unwrap();
    let parsed: ParsedPkcs12 = pkcs12.parse("mypass").unwrap();

    // panic
    //(&parsed.cert).public_key().unwrap().dsa().unwrap();
    //encrypt data and decrypt:
    let publickey: Rsa<Public> = (&parsed.cert).public_key().unwrap().rsa().unwrap();
    let data = b"foobar";
    println!("original string is foobar");
    let mut buf = vec![0; publickey.size() as usize];
    let encrypted_len = &publickey
        .public_encrypt(data, &mut buf, Padding::PKCS1)
        .unwrap();
    println!("encrypted_len is: {}", encrypted_len);
    println!("encrypted data is: {:?}", buf);

    let privatekey = parsed.pkey;
    let rsaprikey: Rsa<Private> = privatekey.rsa().unwrap();
    let mut dec_result = vec![0; rsaprikey.size() as usize];
    let len = rsaprikey
        .private_decrypt(&buf, &mut dec_result, Padding::PKCS1)
        .unwrap();
    println!("decrypted data is :{:?}", dec_result);
    println!("decrpted string is : {}", String::from_utf8(dec_result).unwrap());
    println!("-----------------encrypt_data pkcs12 success---------------------");
}

pub fn parse_empty_chain() {
    let der = include_bytes!("../test/keystore-empty-chain.p12");
    let pkcs12 = Pkcs12::from_der(der).unwrap();
    let parsed = pkcs12.parse("cassandra").unwrap();
    assert!(parsed.chain.is_none());
    println!("-----------------parse_empty_chain pkcs12 success---------------------");
}
