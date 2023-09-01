use std::net::TcpStream;
use ssh2::Session;
use crate::network_device::network_device::*;




pub struct BaseConnection<'a> {
    pub device_type: (&'a String, &'a String), // Device type as a tuple of (vendor, os). Example: ('cisco', 'EOS-4.14.5M.swi')
    pub management_ip: &'a String, // Ip address of the network device to be interfaced with.
    pub management_port: &'a String, // SSH Port in case of non-standard ssh port ergo..not 22.
    pub console_line_number: Option<&'a String>, // Async line number to be used for console connections.
    pub console_server_ip: Option<&'a String>, // Ip address of the console server of target network device.
    pub dns_hostname: &'a String, // DNS hostname of the network device to be interfaced with.
    pub username: &'a String,     // Username to be passed to the device for authentication.
    pub password: &'a String,     // Password to be passed to the device for authentication.
    pub connection_timeout: i32, // If no commands are issued after this many seconds, the connection will be closed.
    pub auth_timeout: Option<i32>, // If no authentication is performed after this many seconds, the connection will be closed.
    pub client_keep_alive: bool, // If true, the client will send keepalive messages to the network device.
    pub client_keep_alive_interval: Option<i32>, // if client_KeepAlive is true, this is the interval in seconds between keepalive messages.
    pub session_timeout: Option<i32>, // Amount of time until session is assumed dead due to lack of network device keepalives.
    pub stream: Option<Session>,    // TcpStream object used for the connection.
    pub default_enter: Option<&'a String>, // Character(s) the represent an enter keypress. Thing \r\n or \n or \r.
    pub response_return: Option<&'a String>, // Character(s) that represent a return in the response from the network device.
    pub session_logging: bool,               // If true log session information to terminal.
    pub session_log_file: Option<&'a String>, // If session_logging is true, this is the file to log to.
    pub session_log_level: Option<&'a String>, // If session_logging is true, this is the level of logging to perform. Possible values are: debug, info, warning, error, critical.
    pub session_log_append_or_truncate: Option<bool>, // If session_logging is true, this is whether to append to the log file or truncate it.
    pub data_encoding: &'a String, // Encoding option for the data stream. Possible values are: ascii, utf-8, utf-16.
}
impl BaseConnection<'_> {
    fn new() -> BaseConnection<'static> {
        BaseConnection {
            device_type: Device::device_type,
            management_ip: Device::management_ip,
            management_port: Device::management_port,
            console_line_number: Device::console_line_number,
            console_server_ip: Device::console_server_ip,
            dns_hostname: Device::dns_hostname,
            username: Device::username,
            password: Device::password,
            connection_timeout: Device::connection_timeout,
            auth_timeout: Device::auth_timeout,
            client_keep_alive: None,
            client_keep_alive_interval: None,
            session_timeout: Some(30),
            stream: Device::stream,
            default_enter: None,
            response_return: None,
            session_logging: None,
            session_log_file: None,
            session_log_level: None,
            session_log_append_or_truncate: Some(false),
            data_encoding: None,
        }
    }
    fn connection(&self) -> Session {
        let tcp = TcpStream::connect(format!("{}:{}", self.management_ip, self.management_port)).unwrap();
        let mut session = Session::new().unwrap();
        session.set_tcp_stream(tcp);
        session.handshake().unwrap();
        session.userauth_password(self.username, self.password).unwrap();
        self.stream = Some(session);
    }
}