// functions5.rs
// Execute `rustlings hint functions5` or use the `hint` watch subcommand for a hint.

fn main() {
    let answer = {
        let s = square(2);
        let c = cube(s);
        c + s
    };
    
    println!("The cube of square of 2 is {}", answer);
}

fn square(num: i32) -> i32 {
    return num * num;
}

fn cube(num: i32) -> i32 {
    return num * num * num;
}
