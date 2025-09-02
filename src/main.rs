use std::io::{self, Write};
use std::env;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use clap::Parser;
use log::{info, error};
use anyhow::Result;

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    full_name: String,
}

#[derive(Parser, Debug)]
#[command(name = "repo-deleter")]
#[command(about = "A CLI tool to delete GitHub repositories")]
struct Args {
    /// GitHub token
    #[arg(long)]
    token: Option<String>,
    
    /// Show what would be deleted without actually deleting
    #[arg(long)]
    dry_run: bool,
    
    /// Skip confirmation prompts
    #[arg(long)]
    yes: bool,
    
    /// Number of repositories per page for pagination
    #[arg(long, default_value = "30")]
    per_page: u32,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    let args = Args::parse();
    
    let token = match args.token.clone().or_else(|| env::var("GITHUB_TOKEN").ok()) {
        Some(t) => t,
        None => get_github_token()?,
    };
    
    run(token, args).await
}

async fn run(token: String, args: Args) -> Result<()> {
    info!("Starting repo-deleter");
    
    let client = reqwest::Client::new();
    
    let repos = fetch_all_repos(&client, &token, args.per_page).await?;
    if repos.is_empty() {
        info!("No repositories found.");
        return Ok(());
    }
    
    info!("Found {} repositories", repos.len());
    print_repos(&repos);
    
    let selected_repos = prompt_selection(&repos)?;
    
    if selected_repos.is_empty() {
        info!("No repositories selected for deletion.");
        return Ok(());
    }
    
    info!("Selected {} repositories for deletion", selected_repos.len());
    
    if args.dry_run {
        info!("DRY RUN MODE - The following repositories would be deleted:");
        for repo in &selected_repos {
            println!("  - {}", repo.name);
        }
        return Ok(());
    }
    
    if !args.yes && !confirm(&selected_repos)? {
        info!("Deletion cancelled by user");
        return Ok(());
    }
    
    for repo in selected_repos {
        match delete_repo(&client, &token, repo).await {
            Ok(()) => info!("Successfully deleted {}", repo.name),
            Err(e) => error!("Failed to delete {}: {}", repo.name, e),
        }
    }
    
    Ok(())
}

async fn fetch_all_repos(client: &reqwest::Client, token: &str, per_page: u32) -> Result<Vec<Repo>> {
    info!("Fetching repositories with pagination (per_page: {})", per_page);
    
    let mut all_repos = Vec::new();
    let mut page = 1;
    
    loop {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);
        headers.insert(USER_AGENT, HeaderValue::from_static("repo-deleter"));
        
        let url = format!(
            "https://api.github.com/user/repos?per_page={}&page={}",
            per_page, page
        );
        
        info!("Fetching page {} of repositories", page);
        
        let response = client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(anyhow::anyhow!("GitHub API request failed: {}", response.status()));
        }
        
        let repos: Vec<Repo> = response.json().await?;
        
        if repos.is_empty() {
            break;
        }
        
        info!("Retrieved {} repositories from page {}", repos.len(), page);
        all_repos.extend(repos);
        page += 1;
    }
    
    info!("Total repositories fetched: {}", all_repos.len());
    Ok(all_repos)
}

fn print_repos(repos: &[Repo]) {
    println!("\nYour repositories:");
    for (i, repo) in repos.iter().enumerate() {
        println!("{}: {}", i + 1, repo.name);
    }
}

fn prompt_selection(repos: &[Repo]) -> Result<Vec<&Repo>> {
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

fn confirm(repos: &[&Repo]) -> Result<bool> {
    println!("\nYou are about to delete the following repositories:");
    for repo in repos {
        println!("  - {}", repo.name);
    }
    
    print!("\nAre you sure you want to delete these repositories? (y/N): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    Ok(input.trim().to_lowercase() == "y" || input.trim().to_lowercase() == "yes")
}

async fn delete_repo(client: &reqwest::Client, token: &str, repo: &Repo) -> Result<()> {
    info!("Deleting repository: {}", repo.name);
    
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("token {}", token))?);
    headers.insert(USER_AGENT, HeaderValue::from_static("repo-deleter"));
    
    let url = format!("https://api.github.com/repos/{}", repo.full_name);
    let response = client.delete(&url).headers(headers).send().await?;
    
    if response.status().is_success() {
        info!("Successfully deleted {}", repo.name);
        println!("Successfully deleted {}", repo.name);
    } else {
        let error_msg = format!("Failed to delete {}: {}", repo.name, response.status());
        error!("{}", error_msg);
        return Err(anyhow::anyhow!(error_msg));
    }
    
    Ok(())
}

fn get_github_token() -> Result<String> {
    print!("Enter your GitHub token: ");
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    Ok(token.trim().to_string())
}
