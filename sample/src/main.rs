use conio_rs::getch;

fn main() {
    println!("Check key codes!");
    loop {
        let code = getch();
        println!("Key:{} Code:{}",code as u8 as char,code);
    }
}
