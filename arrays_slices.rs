fn main() {
    let arr = [1, 2, 3]; // arrays have known length
    let slice = &arr[0..2]; // we don't know length at compile
    borrowing_slice(arr, slice); // functions are created outside of main
}

fn borrowing_slice(arr: [u8; 3], slice: &[u8]) {
    println!("{:?}", arr);
    println!("{:?}", slice); // [1, 2]
    println!("length: {}", slice.len()); // length: 2
    println!("{} {}", slice[0], slice[1]);
}
