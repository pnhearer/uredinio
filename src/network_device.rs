#[allow(dead_code)]

pub mod network_device {
    use ssh2::Session;
    use std::net::TcpStream;
    pub struct Device;
    impl Device {
        pub fn device_type() -> (String, String) {
            let device_type = (String::from("cisco"), String::from("EOS-4.14.5M.swi"));
            device_type
        }
        pub fn management_ip() -> String {
            String::from("some_ip_address")
        }
        pub fn management_port() -> String {
            String::from("22")
        }
        pub fn console_line_number() -> Option<&'static String> {
            None
        }
        pub fn console_server_ip() -> Option<&'static String> {
            None
        }
        pub fn dns_hostname() -> String {
            String::from("cisco1") //TODO: Build example medatadata source. Probably a sqlx database connection or someting similar.
        }
        pub fn username() -> String {
            let x = String::from("someusername"); TODO: Need to apply some log filtering to ensure that password/username are not passed to the log.
            x
        }
        pub fn password() -> String {
            let y = String::from("somepassword"); //TODO: Need to apply some log filtering to ensure that password/username are not passed to the log.
            y
        }
        pub fn connection_timeout() -> i32 {
            32
        }
        pub fn auth_timeout() -> Option<i32> {
            None
        }
        pub fn client_keep_alive() -> Option<bool> {
            None
        }
        fn client_keep_alive_interval() -> Option<i32> {
            None
        }

        pub fn session_timeout() -> Option<i32> {
            None
        }
        pub fn default_enter() -> Option<&'static String> {
            None
        }
        pub fn response_return() -> Option<&'static String> {
            None
        }
        pub fn session_logging() -> Option<bool> {
            None
        }
        pub fn session_log_file() -> Option<&'static String> {
            None
        }
        pub fn session_log_level() -> Option<&'static String> {
            None
        }
        pub fn session_log_append_or_truncate() -> Option<bool> {
            None
        }
        pub fn data_encoding() -> String {
            let encode = String::from("ascii");
            encode
        }
        pub fn connection(username: String, password: String) -> Session {
            let tcp: TcpStream = TcpStream::connect(format!(
                "{}:{}",
                Self::management_ip(),
                Self::management_port()
            ))
            .unwrap();
            let mut session = Session::new().unwrap();
            let username = username;
            let password = password;
            session.set_tcp_stream(tcp);
            session.handshake().unwrap();
            session.userauth_password(&username, &password).unwrap();
            session
        }
    }
}
