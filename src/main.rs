fn main() {
    // scalar data types

    // int
    // we have access from i8 -> i128
    // we default to i32 if not defined "let x = 2;"
    let x: i32 = 2;

    // unsigned
    // cannot use negative
    // as above, access to u8 -> u128
    let y: u32 = 900;

    // a uint technically has a bigger top end because the range of numbers it encompasses is smaller

    // floats
    // defaults to f64
    let floating: f32 = 10.9;

    // bools
     let true_var: bool = true;

    // chars 
    // single character
    let letter: char = 'a';

    
    // compound types

    // tuple
    // fixed length sequence of immutable elements
    let tup = (1, true, 's');

    // print a value from the tuple using its index
    println!("{}", tup.1);
    
    // sequence of mutable elements
    let mut tup = (1, true, 's');

    // change the value of one of the tuple values
    tup.0 = 15;

    // print a value from the tuple using its index
    println!("{}", tup.0);


    // array
    // length of array cannot be altered later
    let mut arr = [1, 2, 3, 4, 5];

    // overwrite a value
    arr[1] = 7;

    // print a value from arr
    println!("{}", arr[1]);

    // explicity define array type
    // variable name -> type of data in array; -> number of elements in array -> array values
    let mut arr_type: [i32; 5] = [1, 2, 3, 4, 5];


    // loop an array and print its values and index
    let mut i = 0;
    let mut test_arr: [&str; 5] = ["one", "two", "three", "four", "five"];

    // prints:
    // 0 = one
    // 1 = two
    // 2 = three
    // 3 = four
    // 4 = five
    while i < 5 {
        println!("{} = {}", i, test_arr[i]);
        i = i + 1;
    }
}
