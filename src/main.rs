use std::collections::BTreeMap;//hashmap but it sorts the values on its own.

fn main(){
    let mut tree:BTreeMap<i32,i32> = BTreeMap::new();
    //insert is to put a key,value set inside the map
    tree.insert(2,20);
    tree.insert(3,30);
    tree.insert(1,10);
    tree.insert(4,40);
    for (key, value) in &tree {
        println!("{}: {}", key, value);}
    let mut treet:BTreeMap<String,BTreeMap<String,i32>> = BTreeMap::new();
    treet.insert("hello".to_string(),BTreeMap::new());
    treet.get_mut("hello").unwrap().//this gives us a mutable reference to the key. and then we can
                                    //use it change it or anything else
        insert("world".to_string(),12);
    treet.insert("Hello".to_string(),BTreeMap::new());
    treet.get_mut("Hello").unwrap().insert("World".to_string(),12);
    for (key, value) in &treet {
        println!("{}: {:?}", key, value);
    }
    treet.entry("hello".to_string()).//check if the there is a value with this key or not. if yes,reference
        or_insert_with(BTreeMap::new).//if know we make this value with that key. and return it as refrence.
        entry("wod".to_string()).
        and_modify(|v| *v +=1).//if it exists we put it in || and then we transform it
        or_insert(323);// it gets the value
        //here or_insert can only have value or expression. it runs whatever function or value be,
        //so better for simple value/expression while the one _with can have closures
        //same as BTreeMap.new -> only ran if value doesn't already exists

        //claude says-> allocating a collection, calling a function, formatting a string, cloning something non-trivial) → always use .or_insert_with()
        //then it must be true
    treet.entry("hell".to_string()).or_insert_with(BTreeMap::new).entry("wold".to_string()).and_modify(|v| *v +=1).or_insert(33);
    treet.entry("helloo".to_string()).or_insert_with(BTreeMap::new).entry("world".to_string()).and_modify(|v| *v +=1).or_insert(233);
    for (key, value) in &treet {
        println!("{}: {:?}", key, value);
    }
}
