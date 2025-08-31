use std::io::{self, Write};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    full_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = get_github_token()?;
    let client = reqwest::Client::new();

    let repos = get_repos(&client, &token).await?;
    if repos.is_empty() {
        println!("No repositories found.");
        return Ok(());
    }

    print_repos(&repos);

    let selected_repos = get_selected_repos(&repos)?;

    if selected_repos.is_empty() {
        println!("No repositories selected for deletion.");
        return Ok(());
    }

    delete_repos(&client, &token, selected_repos).await?;

    Ok(())
}

fn get_github_token() -> Result<String, io::Error> {
    print!("Enter your GitHub token: ");
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    Ok(token.trim().to_string())
}

async fn get_repos(client: &reqwest::Client, token: &str) -> Result<Vec<Repo>, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token)).unwrap());
    headers.insert(USER_AGENT, HeaderValue::from_static("repo-deleter"));

    let repos = client
        .get("https://api.github.com/user/repos")
        .headers(headers)
        .send()
        .await?
        .json::<Vec<Repo>>()
        .await?;

    Ok(repos)
}

fn print_repos(repos: &[Repo]) {
    println!("\nYour repositories:");
    for (i, repo) in repos.iter().enumerate() {
        println!("{}: {}", i + 1, repo.name);
    }
}

fn get_selected_repos(repos: &[Repo]) -> Result<Vec<&Repo>, io::Error> {
    print!("\nEnter the numbers of the repositories you want to delete (comma-separated): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let selected_numbers: Vec<usize> = input
        .trim()
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect();

    let selected_repos: Vec<&Repo> = selected_numbers
        .into_iter()
        .filter_map(|n| repos.get(n - 1))
        .collect();

    Ok(selected_repos)
}

async fn delete_repos(client: &reqwest::Client, token: &str, repos: Vec<&Repo>) -> Result<(), reqwest::Error> {
    println!("\nDeleting selected repositories...");
    for repo in repos {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token)).unwrap());
        headers.insert(USER_AGENT, HeaderValue::from_static("repo-deleter"));

        let url = format!("https://api.github.com/repos/{}", repo.full_name);
        let response = client.delete(&url).headers(headers).send().await?;

        if response.status().is_success() {
            println!("Successfully deleted {}", repo.name);
        } else {
            println!("Failed to delete {}: {}", repo.name, response.status());
        }
    }
    Ok(())
}
