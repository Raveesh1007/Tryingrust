fn main(){
    stack_fn();
    heap_fn();
    update_fn();
}


fn stack_fn() {
    let x = 5;
    let y = 4;
    let c = x + y ;
    println!("Stack Function: The sum of {} and {} is {}", x, y, c);
}

fn heap_fn() {
    let s1 = String::from("Hello,");
    let s2 = String::from("World!");
    let combined = format!("{} {}", s1, s2);
    println!("Heap Function: {}", combined);
}

fn update_fn() {
    let mut s= String::from("Initial String And ");
     println!("Before Update! {}", s);
     println!("Capacity {}, Length{}, Pointer{:p}", s.capacity(), s.len(), s.as_ptr()); 


    s.push_str("Good Morning!");
    println!("After Update! {}", s);
    println!("Capacity {}, Length{}, Pointer{:p}", s.capacity(), s.len(), s.as_ptr());
}

