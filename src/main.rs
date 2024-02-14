fn not_complete_fn(x: usize) -> bool {
    if x < 10 {
        return true;
    }

    todo!("finish this later...")
}

fn main() {
    println!("Hello, world!");
    let x = not_complete_fn(11);
}
