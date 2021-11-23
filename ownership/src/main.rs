fn main() {
    {   // s is not valid here, it's not yet declared
        // s is valid from this point forward
        let s = "hello";
    }   //this scope is now oever, and si s no longer valid

    let mut s = String::from("hello");

    s.push_str(", world!"); //push_str() appends a literal to a string

    println!("{}", s); //this will print hello world
}
