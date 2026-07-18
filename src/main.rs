mod modules;
use serde::{Deserialize,Serialize};
use serde_json::{Result,Value, json};

#[derive(Deserialize,Serialize)]
struct val1{
    name:String,
    count:Vec<u32>
}

fn main() {
    println!("Hello, world!");

    //if i understand properly r# is for raw string. and we end with #. this allows us to easily put
    // " inside of ". as in rust ' is from chars and hence we can't just ' efde"fvc" ' like python
    let string = r#"
    {
        "name" : "Abhinav",
        "count" : [100]
    }"#;
    

    //as here type is val1 not value we can be more through in how the json file must be structured
    //and what values must be in it.
    let v:val1 = match serde_json::from_str(string){
        Ok(v) => {println!("yes");v}
        Err(e) => {panic!("Hell Nah!! {e}");}
    };

    println!{"values are {} and {}",v.name,v.count[0]};
    //                              ^           ^
    //                              |           |
    //                              ---&[value]--


    //it creates a value type json of whats written in the marco[ in {} format
    let string2 = json!({"name":"abhi","count":[1,2,3]});
    
    println!{"values are {} and {},{},{}",string2["name"],string2["count"][0],string2["count"][1],string2["count"][2]};

    // new find what we have seen code before are not json but a type made so we can use json
    // easily. next we use to_string this turns normal struct into actual json string

    let string = val1 {
        name : "freya".to_owned(),
        count: vec!(17),
        };

    let j = serde_json::to_string(&string).unwrap();
    println!{"{}",j};
}
