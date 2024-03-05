/**
 * Lesson 4: Loops.
 */
fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            // Returns 20.
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Labelled loop.
    // Starts with a '.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // While loops.
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // For loops.
    for element in a {
        println!("the value is: {element}");
    }

    // Number within range.
    // .rev() reverses a range of numbers.
    for number in (1..4).rev() {
        println!("{number}!");
    }
}
