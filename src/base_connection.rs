#[allow(dead_code)]

pub mod base_connection {
    use crate::network_device::network_device::{self, *};

    pub struct BaseConnection<'a> {
        pub device_type: (String, String), // Device type as a tuple of (vendor, os). Example: (Arista', 'EOS-4.14.5M')
        pub management_ip: String, // Ip address of the network device to be interfaced with.
        pub management_port: String, // SSH Port in case of non-standard ssh port ergo..not 22.
        pub console_line_number: Option<&'a String>, // Async line number to be used for console connections.
        pub console_server_ip: Option<&'a String>, // Ip address of the console server of target network device.
        pub dns_hostname: String, // DNS hostname of the network device to be interfaced with.
        pub username: String,     // Username to be passed to the device for authentication.
        pub password: String,     // Password to be passed to the device for authentication.
        pub connection_timeout: i32, // If no commands are issued after this many seconds, the connection will be closed.
        pub auth_timeout: Option<i32>, // If no authentication is performed after this many seconds, the connection will be closed.
        pub client_keep_alive: bool, // If true, the client will send keepalive messages to the network device.
        pub client_keep_alive_interval: Option<i32>, // if client_KeepAlive is true, this is the interval in seconds between keepalive messages.
        pub session_timeout: Option<i32>, // Amount of time until session is assumed dead due to lack of network device keepalives.
        pub stream: ssh2::Session,        // TcpStream object used for the connection.
        pub default_enter: Option<&'a String>, // Character(s) the represent an enter keypress. Thing \r\n or \n or \r.
        pub response_return: Option<&'a String>, // Character(s) that represent a return in the response from the network device.
        pub session_logging: bool,               // If true log session information to terminal.
        pub session_log_file: Option<&'a String>, // If session_logging is true, this is the file to log to.
        pub session_log_level: Option<&'a String>, // If session_logging is true, this is the level of logging to perform. Possible values are: debug, info, warning, error, critical.
        pub session_log_append_or_truncate: Option<bool>, // If session_logging is true, this is whether to append to the log file or truncate it.
        pub data_encoding: String, // Encoding option for the data stream. Possible values are: ascii, utf-8, utf-16.
    }

    impl BaseConnection<'_> {
        pub fn new() -> BaseConnection<'static> {
            BaseConnection {
                device_type: Device::device_type(),
                management_ip: Device::management_ip(),
                management_port: Device::management_port(),
                console_line_number: Device::console_line_number(),
                console_server_ip: Device::console_server_ip(),
                dns_hostname: Device::dns_hostname(),
                username: Device::username(),
                password: Device::password(),
                connection_timeout: Device::connection_timeout(),
                auth_timeout: Device::auth_timeout(),
                client_keep_alive: true,
                client_keep_alive_interval: Some(30),
                session_timeout: Some(30),
                stream: network_device::Device::connection(
                    network_device::Device::username(),
                    network_device::Device::password(),
                ),
                default_enter: None,
                response_return: None,
                session_logging: false,
                session_log_file: None,
                session_log_level: None,
                session_log_append_or_truncate: Some(false),
                data_encoding: Device::data_encoding(),
            }
        }
    }
}
