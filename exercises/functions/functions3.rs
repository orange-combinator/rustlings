// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.

fn main() {
    call_me();
}

fn call_me() {
    for i in 0..3 {
        println!("Ring! Call number {}", i + 1);
    }
}
