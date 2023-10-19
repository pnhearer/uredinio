use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};
use ssh2::*;
//use std::io::prelude::*;
use std::time::Duration;


fn main() {
    let ipv4 = std::net::SocketAddr::new(IpAddr::V4("127.0.0.1".parse().unwrap()), 22);
    let tcp = match TcpStream::connect_timeout(&ipv4, Duration::new(5, 0)) {
        Ok(tcp) => tcp,
        _ => return
    };
    // Begin Session build
    let mut session = match Session::new() {
        Ok(session) => session,
        _ => return
    };
    // Takes ownership of TCP stream
    session.set_tcp_stream(tcp);
    // Handshake transport protocol
    match session.handshake() {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e)
    };

    // Authentication
    match session.userauth_password("username", "password") {
        Ok(()) => (),
        Err(e) => println!("Error: {}", e)
    };
    // Create channel, be lazy and use expect.
    let mut channel = session.channel_session().expect("Failed to establish channel");
    // Ask for a Shell
    match channel.shell() {
        Ok(()) => (),
        Err(e) => println!("{}", e)
    }

    //Let's Issue commands to the remote host!
    channel.write_all(b"ls").unwrap();

    //Begin the read loop

    loop {
        let mut rbuff = [0u8;1024];
        match channel.read(&mut rbuff) {
            Ok(0) => break,
            Ok(c) => {
                let slice = &rbuff[0..c];
                match std::str::from_utf8(slice) {
                    Ok(s) => print!("{}",s ),
                    Err(e) => {
                        eprint!("Output was not UTF-8. Instead was {}", e);
                        break
                    }
                }
            },
            Err(e) => {
                println!("Error while reading {}",e );
                break
            }
        }
        };
}


