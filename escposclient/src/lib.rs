mod socket;

#[test]
fn t(){
    assert_eq!(1,1);
}

pub fn send_message(msg : String, address: String, port: u32) {
    socket::send_message(msg, address, port);
}

pub fn add_text(mut msg: String, to_add: String) -> String {
    msg.push_str(to_add.as_str());
    return msg;
}


pub fn cut(mut msg: String) -> String{
    msg.push_str("\x1D");
    msg.push_str("\x56");
    msg.push_str("\x41");
    msg.push_str("\x08");
    msg.push_str("\n");
    msg
}


pub fn all_black(mut msg: String, num: u32) -> String {
    if num > 2400{
        return msg;
    }
    let nL: u8 = (num % 256) as u8;
    let nH: u8 = (num /256) as u8;
    
    msg.push_str("\x1B");
    msg.push_str("\x2A");
    msg.push_str("\x01");
    msg.push(nL as char);
    msg.push(nH as char);
    for _ in 0..num {
        for _ in 0..8 {
            msg.push_str("\x01");
        }
    }
    msg
}
