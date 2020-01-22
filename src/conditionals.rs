pub fn run() {
    let age: u8 = 22;
    let check_id = false;
    let knows_person_of_age = true;

    // if/Else
    if age >= 21 && check_id || knows_person_of_age {
        println!("Bartender: What would you like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry you have to leave");
    } else {
        println!("Braternder:  I'll need to see your ID");
    }

    //shorthand if

    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of Age: {}", is_of_age);
}
