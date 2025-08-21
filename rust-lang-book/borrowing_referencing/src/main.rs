// & is used for referencing
// Rules for References:
// 1 - At any given time, you can have either one mutable reference or any number of immutable references.
// 2 - References must always be valid.

fn main() {
    let mut s = String::from("hello");

    change(&mut s);

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    // cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{r1}, {r2}, and {r3}");
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

