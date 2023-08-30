use std::{net::UdpSocket, io::{stdin, Read}};
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use magic_crypt::{new_magic_crypt, MagicCryptTrait};


fn main() {

    let mut args = std::env::args();

    match &args.nth(1).as_deref() {
        Some(key) => {
            let addr = nix::ifaddrs::getifaddrs().unwrap()
                .find(|x| {x.address.is_some() && x.broadcast.is_some()});
            match addr {
                Some(addr) => {
                    let broadcast_str = addr.broadcast.unwrap().to_string();
                    let broadcast = broadcast_str.split_once(":").unwrap();
                    let socket = UdpSocket::bind("0.0.0.0:8980").expect("can't bind");
                    socket.set_broadcast(true).expect("can't broadcast");
                    socket.connect(format!("{}:8979", broadcast.0)).expect("can't connect");

                    let mc = new_magic_crypt!(key, 256);

                    let mut content = String::new();
                    stdin().read_to_string(&mut content).expect("can't read from stdin");

                    let encrypted = mc.encrypt_str_to_bytes(content);
                    socket.send(&encrypted).expect("can't send");
                }
                None => {
                    eprintln!("can't broadcast")
                }
            }
        }
        None => {
            let socket = UdpSocket::bind("0.0.0.0:8979").expect("can't bind");
            let key = gen_key(4);
            eprintln!("{}", &key);

            loop {
                let mut buf = [0; 1024 * 1024];

                let (size, src) = socket.recv_from(&mut buf).expect("failed to receive");

                let mc = new_magic_crypt!(&key, 256);

                let read = &buf[0..size];

                match mc.decrypt_bytes_to_bytes(&read) {
                    Ok(content) => {
                        print!("{}", String::from_utf8_lossy(&content));
                        break;
                    }
                    Err(e) => {
                        eprintln!("{} {}", src, e);
                    }
                }
            }
        }
    }
}

fn gen_key(n: usize) -> String {
    let r = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .collect::<Vec<_>>();
    String::from_utf8(r).unwrap()
}
