fn main(){
    let greeting  = String::from("Hello World");
    print!("{}", greeting);

    let char1 = greeting.chars().nth(1000);

    match char1 {
        Some(c) => print!("{}", c),
        None => print!("No character at index 1000"),

    }

    print!("char1 :{:?}", char1);


}
