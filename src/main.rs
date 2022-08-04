fn main() {
    // * data types in rust
    // ? two main data type subsets in rust: scalar and compound

    // ** scalar types 
    // scalar types represent a single value - integer types, float types, booleans and characters

    // * integers
    // can be stored in 8, 16, 32, 64, or 128 bits
    // can be signed or unsigned (signed can be positive or negative, unsigned are only positive)
    // signed variants can store numbers from -(2^(n - 1)) to 2^(n - 1) - 1 inclusive, where n is the number of bits that variant uses. 
    // So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. 
    // Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

    let negative_int: i8 = -4;
    let positive_int: u8 = 4;

    // * floating point numbers
    // rust has two primitive data types to store floats: f32 and f64
    // the default is f64
    // all floating point types are signed

    // Floating-point numbers are represented according to the IEEE-754 standard. 
    // The f32 type is a single-precision float, and f64 has double precision.

    let float_1 = 2.0; // f64
    let float_2: f32 = 3.0; // f32

    // * numerical operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 20;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // integer division rounds DOWN to the nearest integer, results in 0

    // remainder
    let remainder = 43 % 5;

    // * booleans
    // stored in one bit
    // booleans can be true or false
    let t = true;
    let f: bool = false; // with explicit type annotation

    // * chars
    // specified with single quotes
    // only holds on character
    let c = 'c';
    let uppercase: char = 'C';

    // ** compound types
    // group multiple values into one type - rust only has tuples and arrays

    // * tuples
    // general way of grouping together values with a variety of types into one type
    // tuples have fixed length - once defined they cannot grow or shrink in size
    let example_tuple: (i8, f64, u32) = (-100, 5.7, 2);

    // we can use pattern matching to destructure elements from tuples
    let (first, second, third) = example_tuple;
    println!("The value of the second item in the tuple is: {second}");  

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access
    let negative_one_hundred = example_tuple.0;
    println!("The value of the first item in the tuple is: {negative_one_hundred}");

    // * arrays
    // another way to have a collection of multiple values
    // all items must be of the same type
    // also has a fixed length
    let arr = [1,2,3,4,5];
    let arr_2: [i32; 5] = [1,2,3,4,5]; // with explicit type annotation

    // you can also initialize an array to contain the same value for each element by specifying the initial
    //  value, followed by a semicolon, and then the length of the array in square brackets
    let example_arr = [3; 5];
    // this is equivalent to writing
    // let example_arr = [3,3,3,3,3];

    // you can access array values using bracket notation and the index, like so
    let arr_val_1 = arr[0];



}
