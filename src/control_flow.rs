pub fn _control_flow() {
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let condition = true;
    let number2 = if condition { 5 } else { 6 };
    println!("The value of number is: {number2}");
    let number3 = 6;
    if number3 % 4 == 0 {
        println!("number is divisible by 4");
    } else if number3 % 3 == 0 {
        println!("number is divisible by 3");
    } else if number3 % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
pub fn _loops() {
    loop {
        println!("again!");
        break;
    }
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    let mut number4 = 3;

    while number4 != 0 {
        println!("{number4}!");

        number4 -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
    let a2 = [10, 20, 30, 40, 50];

    for element in a2 {
        println!("the value is: {element}");
    }
    for number5 in (1..4).rev() {
        println!("{number5}!");
    }
    println!("LIFTOFF!!!");
}
