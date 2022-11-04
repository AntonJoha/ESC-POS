mod socket;

fn main() {
    socket::send_message("Hello world\n".to_string(), "192.168.0.157".to_string(), 9100);
}
