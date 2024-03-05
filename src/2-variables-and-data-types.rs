use core::arch;

/**
* Lesson 2: Variables and Data Types.
*/
fn main() {
    // Immutable variables.
    let immutable_variable = 5;
    println!("The value of immutable_variable is {immutable_variable}!");
    // immutable_variable = 6;
    // `immutable_variable` cannot be reassigned.
    // println!("The value of immutable_variable is {immutable_variable}!");

    // Mutable variables.
    let mut mutable_variable = 10;
    println!("The value of mutableVariable is {mutable_variable}!");
    mutable_variable = 20;
    println!("The value of mutable_variable after re-assignment is {mutable_variable}!");

    // Constants.
    const CONSTANT: i32 = 10_000;
    println!("The value of constant is {}!", CONSTANT); // Another way of concatenation.

    // Shadowing
    // Shadows 5 until the scope i.e main() ends.
    let immutable_variable = 50;
    {
        // Scope 1.
        // This is a scope, just like in JS or TS or Solidity.
        {
            // Scope 2.
            // You can shadow immutable and mutable variables, but not constants.
            // Shadows 5 and 50.
            let immutable_variable = 500;
            let mutable_variable = 1_000;
            println!(
                "The new values for immutable_variable and mutable_variable in this scope 2 are, {} and {}",
                immutable_variable, mutable_variable
            )
        }

        println!(
            "The new values for immutable_variable and mutable_variable in this scope 1 are, {} and {}",
            immutable_variable, mutable_variable
        )
    }

    // Data Types:
    //  Scalar Types:
    //      Integer.
    //      Boolean.
    //      Floats.
    //      Characters.
    // Compound Types:
    //      Tuples.
    //      Arrays.
    {
        // Scalar Types.
        let default_integer = 25; // i32, default.
        let signed_integer: i32 = -200;
        let unsigned_integer: u32 = 200;
        let architecture_integer: isize = 300;

        // This is weird, but it works. `expression_integer` is 400.
        let expression_integer: u32 = {
            let f: u32 = 200;
            200 * 2
        };

        let default_float = 1.01; // f64, default.
        let floats: f32 = 23.3; // Single precision float.
        let floats2: f64 = 3.00; // Double precision float.

        let truncated = -5 / 3; // Returns -1
        println!("{}", truncated);
        let truncated = 5 as f64 / 3 as f64; // Returns decimal parts.
        println!("{}", truncated);

        let my_bool = true;
        let my_explicit_bool: bool = false;

        // Chars are in single quotes, ''.
        // Strings are in double quotes, "".
        let my_character = 'A';
        let my_explicit_character: char = 'B';
        let my_emoji: char = '🙂';
    }

    {
        // Compound Types.
        // Tuples.
        let tuple: (u8, i64, f64) = (1, 2, 5.0);
        // Retrieve tuples.
        let (x, y, z) = tuple;
        println!("x >> {}, y >> {}, z >> {}", x, y, z);
        println!("Index 0 >> {}", tuple.0);

        // Arrays.
        let my_array = [1, 2, 3, 4, 5];
        let another_array: [i32; 4] = [90, 100, 110, 120];
        let array_with_the_same_values: [u8; 3] = [4; 3];

        let first = another_array[0];
        let second = another_array[1];
    }

    // Functions.
    {
        // My first custom function.
        fn my_first_custom_function() {
            println!("I'm writing Rust 🦀!");
        }

        fn another_function_that_prints_a_square(number: u64) {
            let square: u64 = number * number;
            println!("The square of {} is {}", number, square);
        }

        my_first_custom_function();
        another_function_that_prints_a_square(120);

        fn five() -> i32 {
            5
        }

        fn plus_one(num: i32) -> i32 {
            num + 1
        }

        fn function_returning_multiple_values(num: i32) -> (i32, i64) {
            (num * 2, (num * 4) as i64)
        }

        let num: i32 = five();
        let (x, y) = function_returning_multiple_values(num);
        println!("{}, {}, {}, {}", num, plus_one(num), x, y);
    }

    {
        fn return_true_if_greater_than_60(num: u128) -> bool {
            // () are optional, preferrable not removed.
            // if (x > y) x
            // if x > y ✔
            if num > 60 {
                true
            } else {
                false
            }
        }

        println!("{}", return_true_if_greater_than_60(50));
        println!("{}", return_true_if_greater_than_60(500));

        let number: u32 = if 5 > 4 { 5 } else { 4 };
    }
}
