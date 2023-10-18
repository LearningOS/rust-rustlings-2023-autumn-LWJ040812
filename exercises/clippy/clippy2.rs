// clippy2.rs
// 
// Execute `rustlings hint clippy2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let mut res = 42;
    let option = Some(12);
    match option {
        Some(x) => {
            res += x;
        }
        None => {
            // 处理 Option 为 None 的情况（可选）
        }
    }
    println!("{}", res);
}
