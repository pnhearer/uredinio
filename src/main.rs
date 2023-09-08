use std::io::*;

#[allow(dead_code)]
use crate::base_connection::base_connection::BaseConnection;

mod base_connection;
mod network_device;

fn main() {
    let device = BaseConnection::new();
    if device.stream.authenticated() {
        println!("Session authenticated");
    } else {
        println!("Session not authenticated");
    }
    let mut channel = device.stream.channel_session().unwrap();
    channel.exec("pwd").unwrap();
    let mut return_from_shell = String::new();
    channel.read_to_string(&mut return_from_shell).unwrap();
    println!("{}", return_from_shell);
}
