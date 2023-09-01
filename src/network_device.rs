


pub struct device;
impl device {
    fn device_type() -> (&'static String, &'static String) {
        ("cisco", "EOS-4.14.5M.swi")    
    }
    fn management_ip() -> &'static String {
        "192.168.1.1"
    }
    fn management_port() -> &'static String {
        "22"
    }
    fn console_line_number() -> Option<&'static String> {
        None
    }
    fn console_server_ip() -> Option<&'static String> {
        None
    }
    fn dns_hostname() -> &'static String {
        "cisco1"
    }
    fn username() -> &'static String {
        "admin"
    }
    fn connection_timeout() -> Option<i32> {
        None
    }
    fn auth_timeout() -> Option<i32> {
        None
    }
    fn client_keep_alive() -> Option<bool> {
        None
    }
    fn client_keep_alive_interval() -> Option<i32> {
        None
    }

    fn session_timeout() -> Option<i32> {
        None
    }
    fn stream() -> Option<TcpStream> {
        None
    }
    fn default_enter() -> Option<&'static String> {
        None
    }
    fn response_return() -> Option<&'static String> {
        None
    }
    fn session_logging() -> Option<bool> {
        None
    }
    fn session_log_file() -> Option<&'static String> {
        None
    }
    fn session_log_level() -> Option<&'static String> {
        None
    }
    fn session_log_append_or_truncate() -> Option<bool> {
        None
    }
    fn data_encoding() -> &'static String {
        "ascii"
    }
    
}