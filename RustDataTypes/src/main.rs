use std::io;

fn main() {
    let array = [1, 2, 3, 4, 5];

    println!("The array is {:?}", array);
    println!("The first element of the array is {}", array[0]);

    let mut index = String::new();

    println!("Enter the index of the array you want to access: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    match index.trim().parse::<usize>() {
        Ok(index) => {
            if index < array.len() {
                let element = array[index];

                println!("The element at index [{}] is {}", index, element);
            } else {
                println!("Index out of bounds");
            }
        }
        Err(_) => println!("Invalid index"),
    }
}
