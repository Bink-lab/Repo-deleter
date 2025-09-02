use std::env;
use std::error::Error;
use std::io::{self, Write};
use std::time::Duration;

use clap::Parser;
use futures::StreamExt;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use tokio::time::sleep;

/// Simple GitHub repo deleter — improved safety and UX
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub token. If not provided, will read from GITHUB_TOKEN env var or prompt interactively.
    #[arg(short, long)]
    token: Option<String>,

    /// Don't actually delete; just show what would be deleted.
    #[arg(long)]
    dry_run: bool,

    /// Answer yes to confirmation prompts (non-interactive).
    #[arg(long, short = 'y')]
    yes: bool,

    /// Include forked repositories when listing/selecting.
    #[arg(long)]
    include_forks: bool,

    /// Include archived repositories when listing/selecting.
    #[arg(long)]
    include_archived: bool,

    /// Maximum concurrent delete requests.
    #[arg(long, default_value_t = 4)]
    concurrency: usize,

    /// Page size to fetch from GitHub per request (max 100).
    #[arg(long, default_value_t = 100)]
    per_page: usize,
}

#[derive(Deserialize, Debug)]
struct Repo {
    name: String,
    full_name: String,
    private: Option<bool>,
    archived: Option<bool>,
    fork: Option<bool>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let token = get_token(args.token.as_deref())?;
    let client = reqwest::Client::new();

    let repos = get_all_repos(&client, &token, args.per_page).await?;
    if repos.is_empty() {
        println!("No repositories found for the authenticated user.");
        return Ok(());
    }

    // Filter according to flags
    let filtered: Vec<Repo> = repos
        .into_iter()
        .filter(|r| {
            if !args.include_forks && r.fork.unwrap_or(false) {
                return false;
            }
            if !args.include_archived && r.archived.unwrap_or(false) {
                return false;
            }
            true
        })
        .collect();

    if filtered.is_empty() {
        println!("No repositories matched the current filters.");
        return Ok(());
    }

    print_repos(&filtered);

    let selected_indexes = prompt_selection(filtered.len())?;
    if selected_indexes.is_empty() {
        println!("No repositories selected for deletion.");
        return Ok(());
    }

    let to_delete: Vec<Repo> = selected_indexes
        .into_iter()
        .map(|i| filtered[i].clone())
        .collect();

    confirm_and_delete(&client, &token, to_delete, &args).await?;

    Ok(())
}

fn get_token(cli_token: Option<&str>) -> Result<String, Box<dyn Error>> {
    if let Some(t) = cli_token {
        if !t.trim().is_empty() {
            return Ok(t.trim().to_string());
        }
    }

    if let Ok(env_token) = env::var("GITHUB_TOKEN") {
        if !env_token.trim().is_empty() {
            return Ok(env_token.trim().to_string());
        }
    }

    // Fall back to interactive prompt
    print!("Enter your GitHub token: ");
    io::stdout().flush()?;
    let mut token = String::new();
    io::stdin().read_line(&mut token)?;
    let token = token.trim().to_string();
    if token.is_empty() {
        Err("No GitHub token provided".into())
    } else {
        Ok(token)
    }
}

async fn get_all_repos(
    client: &reqwest::Client,
    token: &str,
    per_page: usize,
) -> Result<Vec<Repo>, reqwest::Error> {
    let mut all: Vec<Repo> = Vec::new();
    let mut page: usize = 1;
    let per_page = per_page.min(100).max(1);

    loop {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("token {}", token)).unwrap(),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("repo-deleter"));

        let url = format!(
            "https://api.github.com/user/repos?per_page={}&page={}",
            per_page, page
        );

        let resp = client.get(&url).headers(headers).send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            eprintln!(
                "Failed to fetch repos (page {}): {} - {}",
                page, status, text
            );
            break;
        }

        let repos_page = resp.json::<Vec<Repo>>().await?;
        let fetched = repos_page.len();
        all.extend(repos_page);

        if fetched < per_page {
            break;
        }
        page += 1;
        // brief pause to be nice to the API for large accounts
        sleep(Duration::from_millis(100)).await;
    }

    Ok(all)
}

fn print_repos(repos: &[Repo]) {
    println!("\nYour repositories:");
    for (i, repo) in repos.iter().enumerate() {
        let vis = if repo.private.unwrap_or(false) {
            "private"
        } else {
            "public"
        };
        let mut tags = Vec::new();
        if repo.fork.unwrap_or(false) {
            tags.push("fork");
        }
        if repo.archived.unwrap_or(false) {
            tags.push("archived");
        }
        let tags = if tags.is_empty() {
            "".to_string()
        } else {
            format!(" [{}]", tags.join(", "))
        };
        println!("{}: {} ({}){}", i + 1, repo.full_name, vis, tags);
    }
    println!();
}

/// Prompt the user for selection. Accepts comma-separated indices and ranges (e.g. 1,3-5,7).
fn prompt_selection(len: usize) -> Result<Vec<usize>, Box<dyn Error>> {
    print!("Enter the numbers of the repositories you want to delete (comma-separated, ranges allowed): ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim();
    if input.is_empty() {
        return Ok(vec![]);
    }
    let mut set = Vec::new();
    for part in input.split(',') {
        let part = part.trim();
        if part.contains('-') {
            let mut pieces = part.splitn(2, '-');
            if let (Some(a), Some(b)) = (pieces.next(), pieces.next()) {
                if let (Ok(start), Ok(end)) = (a.trim().parse::<usize>(), b.trim().parse::<usize>()) {
                    if start == 0 || end == 0 {
                        continue;
                    }
                    for i in start..=end {
                        if i >= 1 && i <= len {
                            set.push(i - 1);
                        }
                    }
                }
            }
        } else if let Ok(n) = part.parse::<usize>() {
            if n >= 1 && n <= len {
                set.push(n - 1);
            }
        }
    }
    // deduplicate and sort
    set.sort_unstable();
    set.dedup();
    Ok(set)
}

async fn confirm_and_delete(
    client: &reqwest::Client,
    token: &str,
    to_delete: Vec<Repo>,
    args: &Args,
) -> Result<(), Box<dyn Error>> {
    println!("\nSelected repositories to be deleted:");
    for r in &to_delete {
        println!("- {}", r.full_name);
    }
    println!();

    if args.dry_run {
        println!("Dry-run mode enabled. No repositories will be deleted.");
        return Ok(());
    }

    if !args.yes {
        println!("Type DELETE (uppercase) to confirm deletion of the above repositories:");
        let mut confirmation = String::new();
        io::stdin().read_line(&mut confirmation)?;
        if confirmation.trim() != "DELETE" {
            println!("Confirmation failed. Aborting.");
            return Ok(());
        }
    } else {
        println!("--yes provided: skipping interactive confirmation.");
    }

    // Perform concurrent deletes with a buffer
    let sem_concurrency = args.concurrency.max(1);
    println!(
        "Deleting {} repositories with concurrency {}...",
        to_delete.len(),
        sem_concurrency
    );

    let token_header = HeaderValue::from_str(&format!("token {}", token))?;
    let futures = futures::stream::iter(to_delete.into_iter().map(|repo| {
        let client = client.clone();
        let token_header = token_header.clone();
        async move {
            let mut headers = HeaderMap::new();
            headers.insert(ACCEPT, HeaderValue::from_static("application/vnd.github.v3+json"));
            headers.insert(AUTHORIZATION, token_header);
            headers.insert(USER_AGENT, HeaderValue::from_static("repo-deleter"));

            let url = format!("https://api.github.com/repos/{}", repo.full_name);
            let resp = client.delete(&url).headers(headers).send().await;
            match resp {
                Ok(r) => {
                    if r.status().is_success() {
                        println!("Deleted: {}", repo.full_name);
                        Ok(())
                    } else {
                        let status = r.status();
                        let body = r.text().await.unwrap_or_default();
                        eprintln!(
                            "Failed to delete {}: {} - {}",
                            repo.full_name, status, body
                        );
                        Err(format!("Failed to delete {}", repo.full_name))
                    }
                }
                Err(e) => {
                    eprintln!("Request error deleting {}: {}", repo.full_name, e);
                    Err(format!("Error deleting {}", repo.full_name))
                }
            }
        }
    }))
    .buffer_unordered(sem_concurrency);

    futures
        .for_each(|res| async {
            if let Err(_e) = res {
                // already printed errors; continue
            }
        })
        .await;

    println!("Done.");

    Ok(())
}

// derive Clone to allow easy movement into async closures
impl Clone for Repo {
    fn clone(&self) -> Self {
        Repo {
            name: self.name.clone(),
            full_name: self.full_name.clone(),
            private: self.private,
            archived: self.archived,
            fork: self.fork,
        }
    }
}
