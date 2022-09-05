use std::sync::Arc;

use std::convert::TryInto;
use std::io::{stdout, Read, Write,stdin};
use std::net::TcpStream;
use std::time::{SystemTime, UNIX_EPOCH};
use rustls::{OwnedTrustAnchor, RootCertStore};
use std::thread;
use std::sync::mpsc;

fn root_creator() -> rustls::ClientConfig{
    let mut root_store = RootCertStore::empty();
    root_store.add_server_trust_anchors(
        webpki_roots::TLS_SERVER_ROOTS
            .0
            .iter()
            .map(|ta| {
                OwnedTrustAnchor::from_subject_spki_name_constraints(
                    ta.subject,
                    ta.spki,
                    ta.name_constraints,
                )
            }),
    );
    rustls::ClientConfig::builder()
    .with_safe_defaults()
    .with_root_certificates(root_store)
    .with_no_client_auth()  
}

fn main() {
    let server_name = "api.binance.com".try_into().unwrap();
    let mut conn = rustls::ClientConnection::new(Arc::new(root_creator()), server_name).unwrap();
    let mut sock = TcpStream::connect("65.9.40.106:443").unwrap();
    let mut tls = rustls::Stream::new(&mut conn, &mut sock);
    let mut words = String::new();
    let (tx, rx) = mpsc::channel();
    let body = concat!(
        "GET /api/v3/time HTTP/1.1\r\n",
        "Host: api.binance.com\r\n",
        "Connection: closed\r\n",
        "Accept-Encoding: identity\r\n",
        "\r\n"
    ).as_bytes();
    loop{
        stdin().read_line(&mut words).expect("Input Error!");
        print!("{}", words.as_str());
        match words.as_str().trim(){  
            "b"=> {
             break;
        },
             _ => {
            words = String::new();
            }
        }
    }
    thread::spawn(move || {
       let c = SystemTime::now().duration_since(UNIX_EPOCH).expect("get_current_unix_err").as_millis();
       tx.send(c).unwrap();
    });
    tls.write_all(body).expect("bad happened");
    let mut plaintext = Vec::new();
    tls.read_to_end(&mut plaintext).unwrap();
    stdout().write_all(&plaintext).unwrap();
    print!("{:?}",rx.recv().unwrap());
}