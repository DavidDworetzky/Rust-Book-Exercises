fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    /* Data Types section. 3.2 */
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    /* Data types. Integer division */

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;
    
    // multiplication
    let product = 4 * 30;
    
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    
    // remainder
    let remainder = 43 % 5;

    /* Data Types Section. 3.2. Booleans.*/
    let t = true;

    let f: bool = false; // with explicit type annotation


    /* char types*/

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    /*tuple types.. Compound types*/

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    /* arrays */

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    /*explicit type declaration*/
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    /*repeating syntax */

    let a = [3; 5];

    /*out of bound array access */

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );


}
