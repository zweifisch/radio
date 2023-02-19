use std::{net::UdpSocket, io::{stdin, Read}};

fn main() {

    let mut args = std::env::args();

    match &args.nth(1).as_deref() {
        Some(input) => {
            let addr = nix::ifaddrs::getifaddrs().unwrap()
                .find(|x| {x.address.is_some() && x.broadcast.is_some()});
            match addr {
                Some(addr) => {
                    let broadcast_str = addr.broadcast.unwrap().to_string();
                    let broadcast = broadcast_str.split_once(":").unwrap();
                    let socket = UdpSocket::bind("0.0.0.0:8980").expect("can't bind");
                    socket.set_broadcast(true).expect("can't broadcast");
                    socket.connect(format!("{}:8979", broadcast.0)).expect("can't connect");

                    let content = 
                        if input == &"-" {
                            let mut content = String::new();
                            stdin().read_to_string(&mut content).expect("can't read from stdin");
                            content
                        } else {
                            input.to_string()
                        };

                    socket.send(content.as_bytes()).expect("can't send");
                }
                None => {
                    eprintln!("can't broadcast")
                }
            }
        }
        None => {
            let socket = UdpSocket::bind("0.0.0.0:8979").expect("can't bind");
            loop {
                let mut buf = [0; 10240];

                let (_size, src) = socket.recv_from(&mut buf).expect("failed to receive");

                println!("{} {}", src, String::from_utf8_lossy(&buf))
            }
        }
    }
}
