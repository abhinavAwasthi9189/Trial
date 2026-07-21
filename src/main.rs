use tokio;
use octocrab::Octocrab;
use std::env;

#[tokio::main]
async fn main()-> Result<(),Box<dyn std::error::Error>> {
    //i don't understand it. just that it is needed for heaby tcp calls
    rustls::crypto::ring::default_provider().install_default().unwrap();
    //in this we make our clint same as request. it takes your github token and returns the result.
    let octocrab = Octocrab::builder().personal_token(env::var("GITHUB_TOKEN")?).build()?;
    let repo = octocrab.repos("abhinavAwasthi9189","IceFinder-ISRO-BAH2026-KalkiCoders").get().await?;
    //                          ^_username                     ^_RepoName
    println!("Stars: {:?}", repo.stargazers_count);
    println!("Description: {:?}", repo.description);

    Ok(())
}
