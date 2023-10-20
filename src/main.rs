use std::io::{Read, Write};
use std::net::{IpAddr, TcpStream};
use ssh2::*;
//use std::io::prelude::*;
use std::time::Duration;


fn main() {

    //GOAL: Our objective is to be able to pass in a device name. Have the information about a device collected from an external data source(I am shooting for SQLx) and then build a device struct with a set of methods specifically applicable to the VENDOR and OS version of the network device.
    //GOAL CONT'd: To acoomplish this we will dynamically allocate traits to the struct to emulate inheritance. Example Cisco 3164 with NXOS is of type "Network_device" and is of type "Cisco" and is of type "NXOS", so it will be given the correct implementation of it's methods.
    //GOAL CONT'd(2): The achieved state is such that we can call myciscoNXdevice1.show_interfaces() and on myjuniperdevice1.show_interfaces() and we can achieve device specific implementations and get the intended output.


    //TODO: We actually have to turn these into functions and create a real flow for the program. Generally we are testing to expand our knowledge of this library.
    //TODO: 1.) Dynamic Auth mechanism selection 2.) Assign host and auth information into a variable  3.) Dynamic crypto negotiation.
    //TODO: 4.) Explore what errors are possible, not sure how to do this except to force breaks intentionally.
    //TODO: Move everything to the library and clean up main
    //TODO: Begin creating the network device object which is manipulable.

    let ipv4 = std::net::SocketAddr::new(IpAddr::V4("a.b.c.d".parse().unwrap()), 22);
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
        Err(e) => {println!("Error: {}", e); return}
    };

    // Authentication
    // TODO: We need to validate which type of auth we want to use. We have a method available to ask which is valid for this SSH Server
    match session.userauth_password("Username", "password") {
        Ok(()) => (),
        Err(e) => {println!("Error: {}", e); return}
    };
    // Create channel, be lazy and use expect.
    // TODO: Explore channel creation associated errors. We need to handle these more gracefully. Possibly a retry.
    let mut channel = session.channel_session().expect("Failed to establish channel");
    // Ask for a Shell
    match channel.shell() {
        Ok(()) => (),
        Err(e) => {println!("Error: {}", e); return}
    }

    //Let's Issue commands to the remote host!
    channel.write_all(b"cat /var/log/bootstrap.log\n").unwrap();
    channel.flush().unwrap();

    //Create a output collector for the upcoming loop.
    let mut rcollector: Vec<u8> = Vec::new();

    //Begin the read loop
    loop {
        // When a command is send to the remote host it is necessarily indeterminable as to
        // what size output will come back to the buffer, thus we must fill the buffer, clear
        // the buffer and repeat et al~.  This read buffer is an array that has no knowledge about
        // itself outside of contents and total length.
        let mut rbuff = [0u8;4096];


        // This match statement is clever to me. It reads. If the returned amount of bytes read
        // in are 0, break the loop. We're done. If the value is of any other amount of bytes
        // returned, we can create a slice of the read buffer indexed from the 0th byth to the cth
        // byte, thus capturing all of the data.
        // from here we create a string from our bytes(assuming they are UTF-8) and print them out)

        match channel.read(&mut rbuff) {
            Ok(0) => break,
            Ok(c) => {
                let mut slice = &rbuff[0..c];
                match std::str::from_utf8(slice) {
                    Ok(s) => { rcollector.extend_from_slice(&slice) ; println!("{}", s)},
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

    for x in rcollector {
    print!("{:?}", x)
    }
}


