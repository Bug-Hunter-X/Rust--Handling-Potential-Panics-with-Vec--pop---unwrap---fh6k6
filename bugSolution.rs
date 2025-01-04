fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    match vec.pop() {
        Some(last_element) => println!("The last element is: {}", last_element),
        None => println!("The vector is empty."),
    }
}