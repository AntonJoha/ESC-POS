
fn main() {

    let s = escposclient::add_text("".to_string(), "Hello\n".to_string());
    //let s = escposclient::cut(s);
    let s = escposclient::all_black(s, 100);
    escposclient::send_message(s, "192.168.0.157".to_string(), 9100);
}
