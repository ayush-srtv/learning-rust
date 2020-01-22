pub fn run() {

    let mut hello = String::from("Hello ");

    //Get length
    println!("Length: {}", hello.len());

    //push char
    hello.push('W');

    //Push string
    hello.push_str("orld");

    //capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Is Empty: {}", hello.is_empty());

    //contains
    println!("Contains 'World' {}", hello.contains("World"));

    //replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace

    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assetion testing
    assert_eq!(10, s.capacity());

    println!("{}", s);



    println!("{}", hello);

}