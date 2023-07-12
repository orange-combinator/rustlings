// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.


fn main() {
    // let a = ???
    let a = [42; 101];

    println!("Created `a` using `let a = [42; 101];`");

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }

    println!("The length of a = {}", a.len());
    println!("a[0] = {}", a[0]);
}
