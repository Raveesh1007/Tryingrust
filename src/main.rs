pub trait Summary {
    fn summarize(&self) -> String;    
}
struct User {
    name: String,
    age: u32,
}

impl Summary for User {


fn main (){
    let user = User {
        name : String::from("Raveesh"),
        age : 25,
    };
    println!("{}", user.summarize());
}