mod socket;

#[test]
fn t(){
    assert_eq!(1,1);
}

pub fn new_print() -> Vec<u8> {
    vec!(0x1B, 0x40)
}

pub fn send_message(msg : Vec<u8>, address: String, port: u32) {
    socket::send_message(msg, address, port);
}

pub fn add_text(mut msg: Vec<u8>, to_add: String) -> Vec<u8> {
    for i in to_add.as_bytes() {
        msg.push(i.clone());
    }
    msg
}


pub fn cut(mut msg: Vec<u8>) -> Vec<u8>{
    msg.push(0x1D);
    msg.push(0x56);
    msg.push(0x41);
    msg.push(0x08);
    msg.push('\n' as u8);
    msg
}

//https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=128
pub fn barcode(mut msg: Vec<u8>, input: String, print_mode: u8) -> Vec<u8> {
    
    msg.push(0x1D);
    msg.push(0x6B);
    msg.push(print_mode); // Printing mode UPC -A 
    for i in input.as_bytes() {
        msg.push(i.clone());
    }
    msg.push(0x00);
    msg
}



//https://reference.epson-biz.com/modules/ref_escpos/index.php?content_id=88
//TODO FIX THIS AS IT DOESN'T WORK
pub fn all_black(mut msg: Vec<u8>, num: u32) -> Vec<u8> {
    if num > 2400{
        return msg;
    }
    let n_l: u8 = (num % 256) as u8;
    let n_h: u8 = (num /256) as u8;
    
    msg.push(0x1B);
    msg.push(0x2A);
    msg.push(0x01);
    msg.push(n_l as u8);
    msg.push(n_h as u8);
    for _ in 0..num {
        for _ in 0..240 {
            msg.push(0x01);
        }
    }
    msg.push(0x00);
    msg
}



