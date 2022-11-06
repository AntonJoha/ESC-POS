
fn main() {
    let s = escposclient::new_print();
    let s = escposclient::add_text(s, "MORSLA MORSLA MORSLA\n".to_string());
    //let s = escposclient::all_black(s, 1);
    //let s = escposclient::barcode(s, "HEJ".to_string(), 0x04);

    let s = escposclient::cut(s);
    escposclient::send_message(s, "192.168.0.157".to_string(), 9100);
}
