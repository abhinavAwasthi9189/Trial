mod modules;
use serde::{Deserialize,Serialize};
use serde_json::{Result,Value};

#[derive(Deserialize,Serialize)]
struct val1{
    name:String,
    count:u32
}

fn main() {
    println!("Hello, world!");
    let string = r#"
    {
        "name" : "Abhinav",
        "count" : 100
    }"#;
    
    println!("Hello, world!");
    let v:Value = match serde_json::from_str(string){
        Ok(v) => {println!("yes");v}
        Err(e) => {panic!("Hell Nah!! {e}");}
    };

    println!("Hello, world!");
    println!{"values are {} and {}",v["name"],v["count"]};

}
