use crate::common::input;

pub fn exec() {
    println!("start dice");
    loop {
        let select_dice = rand::random_range(1..6);
        println!("dice is {}", select_dice);
        match input("continue(y, n) > ").as_str() {
            "y" => continue,
            _ => {
                println!("end dice");
                return;
            }
        }
    }
}
