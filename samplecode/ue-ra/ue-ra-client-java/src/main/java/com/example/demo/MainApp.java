package com.example.demo;

import org.bouncycastle.util.encoders.Base64;
import org.springframework.boot.autoconfigure.SpringBootApplication;

import javax.net.ssl.*;
import java.io.*;
import java.net.Socket;
import java.security.*;
import java.security.cert.Certificate;
import java.security.cert.CertificateException;
import java.security.cert.X509Certificate;
import java.security.spec.PKCS8EncodedKeySpec;
import java.util.List;


@SpringBootApplication
public class MainApp {
	public static void main(String[] args) {

		TrustManager[] trustAllCerts = new TrustManager[] {
			new X509TrustManager() {
				public X509Certificate[] getAcceptedIssuers() {
					return new X509Certificate[0];
				}
				public void checkClientTrusted(X509Certificate[] certs, String authType) {}
				public void checkServerTrusted(X509Certificate[] certs, String authType) throws CertificateException{
					List<Byte> byteArray;
					byteArray = Utils.convertByte();
					CertData certData = Cert.unmarshalByte(byteArray);

					byte[] attnReportRaw = Cert.verifyCert(certData.payload);
					Cert.verifyAtteReport(attnReportRaw,certData.pub_k);
				}
			}
		};

		HostnameVerifier hv = new HostnameVerifier() {
			public boolean verify(String hostname, SSLSession session) { return true; }
		};
		try{
			File crtFile = new File("./../cert/client.crt");
			List<X509Certificate> certificateChain = PemReader.readCertificateChain(crtFile);

			PrivateKey key = getPemPrivateKey("./../cert/client.pkcs8","EC");

			KeyStore keyStore = KeyStore.getInstance("JKS");
			keyStore.load(null, null);
			keyStore.setKeyEntry("key", key, "".toCharArray(), certificateChain.stream().toArray(Certificate[]::new));

			KeyManagerFactory keyManagerFactory = KeyManagerFactory.getInstance("SunX509");
			keyManagerFactory.init(keyStore, "".toCharArray());

			SSLContext sc = SSLContext.getInstance("SSL");
			sc.init(keyManagerFactory.getKeyManagers(), trustAllCerts, new SecureRandom());

			SSLSocketFactory sf = sc.getSocketFactory();

			Socket s = sf.createSocket("127.0.0.1", 3443);

			// 向客户端回复信息
			DataOutputStream out = new DataOutputStream(s.getOutputStream());
			System.out.print("请输入:\t");
			// 发送键盘输入的一行
			String str = new BufferedReader(new InputStreamReader(System.in)).readLine();
			out.writeUTF(str);

			BufferedReader in = new BufferedReader(new InputStreamReader(s.getInputStream()));
			String x = in.readLine();
			System.out.println(x);

			out.close();
			in.close();
		}catch (Exception e){
			System.out.println(e.toString());
			return;
		}
		System.out.println("loadKeyStore success");
	}

	public static PrivateKey getPemPrivateKey(String filename, String algorithm) throws Exception {
		File f = new File(filename);
		FileInputStream fis = new FileInputStream(f);
		DataInputStream dis = new DataInputStream(fis);
		byte[] keyBytes = new byte[(int) f.length()];
		dis.readFully(keyBytes);
		dis.close();

		String temp = new String(keyBytes);
		String privKeyPEM = temp.replace("-----BEGIN PRIVATE KEY-----\n", "");
		privKeyPEM = privKeyPEM.replace("-----END PRIVATE KEY-----", "");
		//System.out.println("Private key\n"+privKeyPEM);

		Base64 b64 = new Base64();
		byte [] decoded = b64.decode(privKeyPEM);

		PKCS8EncodedKeySpec spec = new PKCS8EncodedKeySpec(decoded);
		KeyFactory kf = KeyFactory.getInstance(algorithm);
		return kf.generatePrivate(spec);
	}
}
