pub fn run() {

    let mut count = 0;
    
    //Infine Loop
    // loop {
    //     count +=1;
    //     println!("Numnber: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }


    //While Loop (FizzBuzz)

    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("fizzbuzz");
    //     } else if count % 3 == 0 {
    //         println!("fizz")
    //     } else {
    //         println!("{}", count);
    //     }

    //     //INC
    //     count +=1;

    // }

    //For Range

    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz")
        } else {
            println!("{}", x);
        }
    }

}