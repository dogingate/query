pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id {
        println!("yes,what would you like to drink?");
    } else if age < 21 && check_id {
        println!("sorry,you have to leave");
    } else {
        println!("I'll need tor see your ID");
    }
    //shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("{}", is_of_age);
}
