
fn main() {
    for i in 1..=5 {
        println!("Iteration: {}", i);
    }

    let mut count = 0;
    while count < 3 {
        println!("While loop count: {}", count);
        count += 1;
    }
}
