use searching::binary_search::binary_search;

fn main() {
    let idx = binary_search(&6, &vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    match idx {
        Some(i) => {
            println!("The index is {}", i);
        },
        None => {
            println!("Not found");
        }
    }
}
