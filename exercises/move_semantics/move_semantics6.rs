// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(&data);
    println!("{}", last_char);

    let uppercased_data = string_uppercase(data);
    println!("{}", uppercased_data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: String) -> String {
    let uppercased_data = data.to_uppercase();

    println!("{}", uppercased_data);

    uppercased_data
}
