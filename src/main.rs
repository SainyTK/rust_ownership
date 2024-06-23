// Rules of ownership
// 1. Each value has an "owner".
// 2. There can only be ONE owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn mutable_string() {
    println!("Mutable string...");
    // let s = "hello"; can't be mutated.
    // let mut s = String::from(); stores data in "heap" rather than "stack" as it is a complex data type.
    let mut s = String::from("hello");
    s.push_str(", world~");
    println!("{s}");
}

fn multiple_variables_simple() {
    println!("Multiple variables simple...");
    // For simple data types (number, bool, string literal), data will be copied
    let x = 5;
    let y = x; // Create a copy of 5 to y

    println!("x = {x}, y = {y}");
}

fn multiple_variables_complex() {
    println!("Multiple variables complex...");
    // For complex data types, "data pointer" will be moved instead

    // A complex variable composes of { ptr, len, capacity }. 
    // This group of data is stored on the stack.
    // The actual data ("hello") is store on heap
    let s1 = String::from("hello");

    // s2 copies pointer of s1 and stored on the stack
    // After this line, s1 is no longer valid. Rust does this to prevent "double free error"
    // We call this reference "move"
    let s2 = s1;
    
    // println!("{s1}, world"); // This will cause an error
    println!("{s2}, world");
}

fn using_clone() {
    println!("Clone complex data...");
    let s1 = String::from("hello");

    // If we use "clone", the data on the heap is copied (created)
    // That's why this operation is expensive
    let s2 = s1.clone();

    // s1 will still be valid as the reference is not moved
    println!("s1 = {s1}, s2 = {s2}");
}

fn print_string(value: String) {
    println!("This is a string:  {value}");
}

fn print_integer(value: i32) {
    println!("This is an integer: {value}");
}

fn passing_function_params() {
    println!("Passing function params...");

    let s = String::from("hello");
    // Passing a complex data will move ownership to the function param (value: String)
    // At the end of function calling, value: String is free
    // That's why we can no longer use s in this scope
    print_string(s);

    // Passing a simple data will copy the data
    // x in this scope is still valid
    let x = 5;
    print_integer(x);

    // This line will cause an error of invalid s
    // println!("Check validity of s: {s}");

    // This line won't cause any error since x is still valid
    println!("Check validity of x: {x}");
}

fn takes_and_gives_back(value: String) -> String {
    value
}

fn function_with_return() {
    println!("Function with return...");

    let s = String::from("hello");
    let s2 = takes_and_gives_back(s);

    // s is no longer valid as it moves to "value: String" in the function
    // The function returns the ownership back to s2
    println!("Getting back s and store in s2: {s2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn using_reference() {
    println!("Using reference...");
    // To avoid passing and returning complex data back and forth, we can use reference instead

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    // s1 ownership won't be moved in this case. So, it's still valid in this scope
    // In detail, &s1 holds pointer of s1. Passing it means s: &String => &s1 (copying s1 pointer)
    println!("The length of '{s1}' is {len}.");

    // This process is called "borrowing"
}

fn change(s: &mut String) {
    s.push_str(", world");
}

fn using_mutable_reference() {
    println!("Using mutable referance...");

    let mut s = String::from("hello");

    change(&mut s);

    println!("The final result is: {s}");
}

fn rules_of_mutable_references() {
    println!("Rules of mutable references...");

    // We CANNOT create multiple mutable references at one time
    // For example, if we have "let r1 = &mut s";
    // We cannot have "let r2 = &mut s"; before using r1

    let mut s = String::from("hello");

    // Create a reference r1 to s
    // let r1 = &mut s;
    // This occurs a problem as r2 refers to s before s1 is being used
    // let r2 = &mut s;
    // When printing r1 here, it causes an error.
    // println!("{}, {}", r1, r2);

    // Solution: Reorder referencing
    let r1 = &mut s;
    println!("r1: {}", r1);
    let r2 = &mut s;
    println!("r2: {}", r2);

    // Immutable reference isn't restricted with this rule
    // We can have another reference before using the first one
    let r1 = &s;
    let r2 = &s;
    println!("r1: {}, r2: {}", r1, r2);

    // The reason why Rust does this because it prevents "data races"
    // Data races occurs when multiple mutable references point to the same address
    // They may change data simultanouesly and introduced unknown errors, which are hard to diagnose at runtime
    // That's why we can have multiple immutable references at one time but not mutable references.
}

// This function is dangle as it returns reference of a string.
// The s variable will be dropped after the end of function.
// Hence, the returned reference points to no value
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

// This function is not dangle as returning the complex data like this won't drop s. It will transfer its ownership to any variables receive this function result
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}  

fn about_dangle() {
    println!("About dangle...");

    let result = no_dangle();
    println!("No dangle: {result}");
}

fn using_slice() {
    println!("Using slice...");
    let s = String::from("hello world");

    let len = s.len();

    let slice1 = &s[0..5];
    let slice2 = &s[..5];

    let slice3 = &s[6..len];
    let slice4 = &s[6..];

    println!("Slice 1: {slice1}");
    println!("Slice 2: {slice2}");
    println!("Slice 3: {slice3}");
    println!("Slice 4: {slice4}");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // Returning the first word of the slice
            return &s[0..i];
        }
        
    }

    // Returning whole slice
    &s[..]
}

fn using_slice_function() {
    println!("Using slice function...");

    let s = String::from("hello world");
    let first = first_word(&s);

    // This will occur some error as s.clear() uses mutable reference
    // Using a mutable reference between (immutable reference and its usage) will cause an error.
    // s.clear(); 

    println!("First word is {first}");
}

fn general_string_slice() {
    println!("General string slice...");

    // &str param can be used for both String and string literal
    let my_string = String::from("hello world");
    let w1 = first_word(&my_string[0..6]);
    let w2 = first_word(&my_string[..]);
    let w3 = first_word(&my_string);

    let my_string_literal = "hello world";
    let wl1 = first_word(&my_string_literal[0..6]);
    let wl2 = first_word(&my_string_literal[..]);
    // string literal is already a slice, so we can just pass it here
    let wl3 = first_word(my_string_literal);

    println!("Output: {w1} {w2} {w3} {wl1} {wl2} {wl3}");
}

fn other_slices() {
    println!("Other slices...");
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    for item in slice {
        println!("{item}");
    }

}

fn main() {
    mutable_string();
    multiple_variables_simple();
    multiple_variables_complex();
    using_clone();
    passing_function_params();
    function_with_return();
    using_reference();
    using_mutable_reference();
    rules_of_mutable_references();
    about_dangle();
    using_slice();
    using_slice_function();
    general_string_slice();
    other_slices();
}
