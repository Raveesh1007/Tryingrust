use std::collections::HashMap;

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32>{
    let mut map: HashMap<_, _> =HashMap::new();
    for(key, value) in vec {
        map.insert(key, value);
    }
    return map;
}

fn main(){
    let vec = vec![(String::from ("Raveesh"), 22 ), (String::from("Raman"), 25)];
    let map = group_values_by_keys(vec);

    println!("{:?}", map);
}