mod calculator;
mod common;
mod dice;
mod todo;

fn main() {
    println!("select module");
    println!("1. calulator");
    println!("2. todo");
    println!("3. dice");
    println!("q is end module");

    let select_number: u8 = match common::input("module > ").parse() {
        Ok(num) => num,
        Err(_) => {
            println!("end module");
            return;
        }
    };
    match select_number {
        1 => calculator::exec(),
        2 => todo::exec(),
        3 => dice::exec(),
        _ => {
            println!("end module");
            return;
        }
    }
}
