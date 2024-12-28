pub fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(&gift_message[0..gift_message.len()]);

    println!("{}", gift_message);
}

pub fn attach_message_to_present(message: &str) {
    println!("The present now has this message: {}", message);
}

