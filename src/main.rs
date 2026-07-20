mod modules;
use serde::Deserialize;
use tokio;


//this is just a struct that stores the info we get back.
#[derive(Deserialize,Debug)]
struct git{
    login:String,public_repos:u32,followers:u32}

fn main() {
    println!("enter the username");
    let user = strput();
    //this is the runtime we prepare. as we don't want to make whole main function async. we create
    //a seprate small one inside of it
    let gt = tokio::runtime::Runtime::new().unwrap();
    
    //here blockon is like let whats inside of me happen[which is async. and then return what we got
    //back to synchronous function.
    let end_value = gt.block_on(async{
        match getGit(&user).await{
            Ok(value) => {value}
            Err(e) => {panic!("Error fetching data: {}", e);}
        }}
    );

    println!("User: {}, Repos: {}, Followers: {}", end_value.login, end_value.public_repos, end_value.followers); 
}

async fn getGit(user:&String) -> Result<git,Box<dyn std::error::Error>>{
    let url = format!("https://api.github.com/users/{}",user);
    //we create a new reqwest client
    let client = reqwest::Client::new();
    //git somehow needs this header to work. i will read of it later. this  fucntion sends it to api
    //and when it gets a json result. it returns a git struct object.
    let value = client.get(&url).header("User-Agent", "rusty-boi").send().await?.json::<git>().await?;
    Ok(value)
}

fn strput() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}
