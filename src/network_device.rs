use std::net::TcpStream;

pub mod network_device {
    pub struct Device;
    impl Device {
        pub fn device_type() -> (&'static String, &'static String) {
            ("cisco", "EOS-4.14.5M.swi")
        }
        pub fn management_ip() -> &'static String {
            "192.168.1.1"
        }
        pub fn management_port() -> &'static String {
            "22"
        }
        pub fn console_line_number() -> Option<&'static String> {
            None
        }
        pub fn console_server_ip() -> Option<&'static String> {
            None
        }
        pub fn dns_hostname() -> &'static String {
            let dns_name: &String = &String::from("cisco1"); //TODO: Build example medatadata source. Probably a sqlx database connection or someting similar.
            dns_name
        }
        pub fn username() -> &'static String {
            let username: &String = &String::from("admin"); //TODO: Need to apply some log filtering to ensure that password/username are not passed to the log.
            username
        }
        pub fn connection_timeout() -> Option<i32> {
            None
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
        pub fn data_encoding() -> &'static String {
            "ascii".
        }
    }
}

