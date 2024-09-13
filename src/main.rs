fn main(){
    let mut name = {String::from("Johny depp")};
    let word2 = find_first_word(&name);
    println!("{}", name);
}

fn find_first_word(name : &String) -> &str{
    let mut index = 0;{
        for (_, i) in name.chars().enumerate(){
            if  i == ' '{
                break;
            }
        } index = index + 1;
    }

    return  &name[..index];
}