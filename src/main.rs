use std::{collections::HashMap, ops::Div};

#[allow(warnings)]
fn main() {
    println!("Hello, world!");

    fn primitive_data_types() {
        // Primitive data types include: int, float, bool char

        /*
            Integers
            Rust has signed and unsigned integers types of different sizes
            Signed integers: i8, i16, i32, i64, i128
            Unsigned integers: u8, u16, u32, u64, u128
        */
        let x: i32 = -42;
        let y: u64 = 100;
        println!("Signed Integer: {}", x);
        println!("Unsigned Integer: {}", y);

        /*
            Floats:
            f32, f64
        */
        let pi: f64 = std::f64::consts::PI;
        println!("Value of pi: {}", pi);

        // Boolean - bool
        let is_snowing: bool = false;
        println!("It is currently snowing. {}", is_snowing);

        // Character Type - char
        let letter: char = 'd';
        println!("First letter in my name is: {}", letter);
    }
    // primitive_data_types();

    fn compound_data_types() {
        // Arrays, Tuples, Slices, Strings (slice strings)

        // Arrays: fixed size collection of homogenous elements
        let numbers: [i32; 5] = [1, 2, 3, 4, 5];
        println!("Number array: {:?}", numbers);
        // This doesn't work because the types aren't homogenous
        // let mix = [1, 2, "apple", true];
        // println!("Mix array: {:?}");

        let fruits: [&str; 3] = ["apple", "banana", "orange"];
        println!("Fruits array: {:?}", fruits);
        println!("Fruits array first element: {}", fruits[0]);
        println!("Fruits array second element: {}", fruits[1]);
        println!("Fruits array third element: {}", fruits[2]);

        // Tuples fixed size collection of heterogeneous elements
        let human: (&str, i32, bool) = ("Dakota", 33, false);
        println!("Human tuple is: {:?}", human);
        let my_mix_tuple: (&str, i32, bool, [i32; 5]) = ("Dakota", 33, true, [1, 2, 3, 4, 5]);
        println!("My mix tuple {:?}", my_mix_tuple);

        // Slices - dynamically sized views into a contiguous sequence of elements
        let number_slices: &[i32; 5] = &[1, 2, 3, 4, 5];
        println!("number slice: {:?}", number_slices);

        let animals_slice: &[&str; 4] = &["Bear", "Pig", "Cow", "Chicken"];
        println!("animal slice: {:?}", animals_slice);

        let book_slice: &[&String; 2] = &[&"IT".to_string(), &"Harry Potter".to_string()];
        println!("book slice: {:?}", book_slice);

        // Strings vs String Slices (&str)
        // String: strings can be mutable. They can grow/shrink. They are owned string types. Allocated on the heap. Slow to access.
        // String Slices: immutable, stored on the stack. Reference to a string

        let mut stone_cold: String = String::from("Hell, ");
        println!("Stone Cold Says: {}", stone_cold);
        stone_cold.push_str("Yeah!");
        println!("Stone Cold says: {}", stone_cold);

        let string: String = String::from("Hello, World!");
        let slice: &str = &string[0..5];
        println!("Slice value: {}", slice);
    }
    // compound_data_types();

    fn functions() {
        fn human_id(name: &str, age: u32, height: u32) {
            println!(
                "My name is {}, I am {} years old, and my height is {} inches.",
                name, age, height
            );
        }
        human_id("Dakota", 33, 72);
    }
    // functions();

    fn expressions_and_statements() {
        // Expression: Anything that returns a value
        // Statement: ANything that does not return a value

        let x = {
            let price = 5;
            let quantity = 10;
            price * quantity
        };
        println!("Result is: {}", x);

        let y = add(4, 6);
        println!("Y is {}", y);
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        fn calculate_bmi(height_in_meters: f64, weight_in_kgs: f64) -> f64 {
            weight_in_kgs.div(height_in_meters * height_in_meters)
        }

        let bmi = calculate_bmi(2.0, 90.25);

        println!("The calculated BMI is {:.2}", calculate_bmi(2.0, 90.25));
    }
    // expressions_and_statements();

    fn error_handling() {
        // Error handling with `Option<T>`
        fn divide(numerator: f64, denominator: f64) -> Option<f64> {
            if denominator == 0.0 {
                None
            } else {
                Some(numerator / denominator)
            }
        }

        let result = divide(10.0, 2.0);

        match result {
            Some(x) => println!("Result is {x}"),
            None => println!("Cannot divide by zero!"),
        }

        // Error handling with `Result<T>`
        fn divide_with_result(numerator: f64, denominator: f64) -> Result<f64, String> {
            if denominator == 0.0 {
                Err("Cannot divide by zero".to_string())
            } else {
                Ok(numerator / denominator)
            }
        }

        match divide_with_result(10.0, 0.0) {
            Ok(result) => println!("Result: {}", result),
            Err(err) => println!("Error: {}", err),
        }
    }
    // error_handling();

    fn vectors() {
        // Vectors, UTF8, Hash Maps

        // Vectors are like dynamic arrays. They can grow/shrink as necessary
        let v: Vec<i32> = Vec::new();
        let new_vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
        let mut new_numbers_vec: Vec<i32> = Vec::new();

        new_numbers_vec.push(42);
        new_numbers_vec.push(52);
        new_numbers_vec.push(62);
        new_numbers_vec.push(72);

        println!("{:?}", new_numbers_vec);

        for number in new_numbers_vec {
            println!("{number}")
        }

        let second: &i32 = &new_vec[1]; // Direct indexing
        println!("The second element in new_vec {:?} is {}", new_vec, second);

        let third: Option<&i32> = new_vec.get(2); // .get() indexing
        match third {
            Some(third) => println!("The third element is {third}"),
            None => println!("There is no third element."),
        }
    }
    // vectors();

    fn utf8_encoding() {
        // Different ways to defined Strings
        let s = "whatever".to_string();
        let s = String::from("whatever");
        let mut s = String::from("foo");

        println!("{}", s);

        // Push a string slice into the String
        s.push_str("bar");

        println!("{}", s);
        // Push a single char into the String
        s.push('!');

        println!("{}", s);

        // Combine Strings with the + operator
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used.
        println!("{s3}");
    }
    // utf8_encoding();

    fn hash_maps() {
        // Collection of Key -> Value pairs
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");

        let score = scores.get(&team_name).copied().unwrap_or(0);

        for (key, value) in &scores {
            println!("{key}: {value}");
        }
    }
    // hash_maps();

    fn structs() {
        struct User {
            active: bool,
            username: String,
            email: String,
            sign_in_count: u64,
        }

        let user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        let mut user2 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };

        println!("User2.email: {}", user2.email);
        user2.email = String::from("anotheremail@example.com");
        println!("User2.email: {}", user2.email);

        fn build_user(email: String, username: String) -> User {
            User {
                active: true,
                username,
                email,
                sign_in_count: 1,
            }
        }

        let user3 = build_user(
            "fakeEmail@example.com".to_string(),
            "someMadeUpUsername".to_string(),
        );

        let user4 = User {
            email: String::from("another@example.com"),
            ..user1
        };

        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
    // structs()
}
