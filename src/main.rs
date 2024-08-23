fn main (){
    let x = String::from ("Hello World");

    takes_ownership (x);

    let s = 5;

    makes_copy(s);
}

fn takes_ownership(x: String){
    println!("{}", x);
}

fn makes_copy(s: i32){
    println!("{}", s);
}