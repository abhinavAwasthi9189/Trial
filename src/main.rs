mod modules;
use serde::{Deserialize,Serialize};
use serde_json::{json};
use std::io::Write;
use std::fs::{self,File};
use std::path::Path;

#[derive(Debug,Clone,Deserialize,Serialize)]
struct Val1{
    name:String,
    count:Vec<u32>
}

//this fucntion takes a user and data and makes a json file of user name and puts the data inside of it
fn writeit(json_str : &str, usr : &str) -> std::io::Result<()>{
    let mut user = String::from(usr);
    user.push_str(".json");
    
    let mut fle = File::create(user)?;
    fle.write_all(json_str.as_bytes())?;
    Ok(())
}

//pretty simple we take a json file and we read it. 
//here dyn means that there is an error but we don't know of what type
//Box helps make it a pointer so there is no issue with compilwr about its size(as we don't know
//errors suize)
fn readit(usr:&str) -> Result<Val1,Box<dyn std::error::Error>>{
    let mut user = String::from(usr);
    user.push_str(".json");

    let data = std::fs::read_to_string(user)?;
    let json:Val1 = serde_json::from_str(&data)?;
    Ok(json)
}

//it is a better function same format as last time just we append and read in it
fn check_and_append_read(json_str:&Val1,usr:&str)-> Result<Vec<Val1>,Box<dyn std::error::Error>>{
    let mut user = String::from(usr);
    user.push_str(".json");
    
    //path function makes string into a path and helps as get a value.Ok gives the value for
    //Vec<Val1>>
    //while ? makes sure error go to the right place.
    if Path::new(&user).exists(){
        let data = std::fs::read_to_string(user)?;
        let json:Val1 = serde_json::from_str(&data)?;
        let mut datas = vec![json];
        datas.push(json_str.clone());
        Ok(datas) 
    }
    else{
        let end = serde_json::to_string(&json_str)?;
        fs::write(user,&end)?;
        Ok(vec![json_str.clone()])
    }


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
    let v:Val1 = match serde_json::from_str(string){
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

    let string = Val1 {
        name : "freya".to_owned(),
        count: vec!(17,12,13),
        };

    let j = serde_json::to_string(&string).unwrap();
    println!{"{}",j};
    //we create a user name for now to make a file about.
    let usr:&str = "abhinav";

    //check for the result.
    match writeit(&j,&usr){
        Ok(()) =>{println!("success");}
        Err(e) => {println!("Error writing to file: {}", e);}
    }

    let end_value = readit(&usr).expect("well it didn't work. same as ussual");
    
    println!("values are {:?}",end_value);

    let string = Val1 {
        name : "Jarvis".to_owned(),
        count: vec!(2,32,42),
        };
    
    let end_value = match check_and_append_read(&string,"abhinav"){
        Ok(a) => {a}
        Err(e) => {panic!("{:?}", e);}
    };

    println!("values are {:?}",end_value);
}
