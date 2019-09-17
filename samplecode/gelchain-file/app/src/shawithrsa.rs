use hex::{self, FromHex};
use openssl::hash::MessageDigest;
use openssl::nid::Nid;
use openssl::pkey::PKey;
use openssl::rsa::{Padding, Rsa};
use openssl::sign::{RsaPssSaltlen, Signer, Verifier};

const INPUT: &'static str =
    "65794a68624763694f694a53557a49314e694a392e65794a7063334d694f694a71623255694c41304b49434a6c\
     654841694f6a457a4d4441344d546b7a4f44417344516f67496d6830644841364c79396c654746746347786c4c\
     6d4e76625339706331397962323930496a7030636e566c6651";

const SIGNATURE: &'static str =
    "702e218943e88fd11eb5d82dbf7845f34106ae1b81fff7731116add1717d83656d420afd3c96eedd73a2663e51\
     66687b000b87226e0187ed1073f945e582adfcef16d85a798ee8c66ddb3db8975b17d09402beedd5d9d9700710\
     8db28160d5f8040ca7445762b81fbe7ff9d92e0ae76f24f25b33bbe6f44ae61eb1040acb20044d3ef9128ed401\
     30795bd4bd3b41eecad066ab651981fde48df77f372dc38b9fafdd3befb18b5da3cc3c2eb02f9e3a41d612caad\
     15911273a05f23b9e838faaf849d698429ef5a1e88798236c3d40e604522a544c8f27a7a2db80663d16cf7caea\
     56de405cb2215a45b2c25566b55ac1a748a070dfc8a32a469543d019eefb47";

//Padding : pkcs1
pub fn rsa_sign() {
    let key = include_bytes!("../test/rsa.pem");
    let private_key = Rsa::private_key_from_pem(key).unwrap();
    let pkey = PKey::from_rsa(private_key).unwrap();

    let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
    assert_eq!(signer.rsa_padding().unwrap(), Padding::PKCS1);
    signer.set_rsa_padding(Padding::PKCS1).unwrap();
    signer.update(&Vec::from_hex(INPUT).unwrap()).unwrap();
    let result = signer.sign_to_vec().unwrap();

    assert_eq!(hex::encode(result), SIGNATURE);

    println!("-----------------rsa sign success,padding:pkcs1---------------------");
}

//Padding : pkcs1
pub fn rsa_verify_ok() {
    let key = include_bytes!("../test/rsa.pem");
    let private_key = Rsa::private_key_from_pem(key).unwrap();
    let pkey = PKey::from_rsa(private_key).unwrap();

    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
    assert_eq!(verifier.rsa_padding().unwrap(), Padding::PKCS1);
    verifier.update(&Vec::from_hex(INPUT).unwrap()).unwrap();
    assert!(verifier.verify(&Vec::from_hex(SIGNATURE).unwrap()).unwrap());

    println!("-----------------rsa verify success,padding:pkcs1---------------------");
}

//Padding : pkcs1_pss
pub fn rsa_sign_verify() {
    let key = include_bytes!("../test/rsa.pem");
    let private_key = Rsa::private_key_from_pem(key).unwrap();
    let pkey = PKey::from_rsa(private_key).unwrap();

    let mut signer = Signer::new(MessageDigest::sha256(), &pkey).unwrap();
    signer.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
    assert_eq!(signer.rsa_padding().unwrap(), Padding::PKCS1_PSS);
    signer
        .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
        .unwrap();
    signer.set_rsa_mgf1_md(MessageDigest::sha256()).unwrap();
    signer.update(&Vec::from_hex(INPUT).unwrap()).unwrap();
    let signature = signer.sign_to_vec().unwrap();

    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
    verifier.set_rsa_padding(Padding::PKCS1_PSS).unwrap();
    verifier
        .set_rsa_pss_saltlen(RsaPssSaltlen::DIGEST_LENGTH)
        .unwrap();
    verifier.set_rsa_mgf1_md(MessageDigest::sha256()).unwrap();
    verifier.update(&Vec::from_hex(INPUT).unwrap()).unwrap();
    assert!(verifier.verify(&signature).unwrap());
    println!("-----------------rsa sign verify success,padding:pss---------------------");
}
