use std::collections::HashMap;

fn main(){
    let mut names = HashMap::new();

    names.insert(String::from ("Raveesh"), 22);
    names.insert(String::from ("Rakesh"), 27);

    let first_name_age = names.get("Raveesh");

    match first_name_age {
        Some(age) => println!("Age of Raveesh is {}", age),
        None => println!("No age found for Raveesh"),   
    }
}