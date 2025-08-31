# Repo Deleter

Repo Deleter is a command-line tool written in Rust that allows you to list and delete your GitHub repositories in bulk. It uses the GitHub API and requires a personal access token with the appropriate permissions.

## Features

- Lists all repositories for the authenticated user
- Allows selection of multiple repositories for deletion
- Deletes selected repositories via the GitHub API

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- A GitHub personal access token with `repo` and `delete_repo` permissions

## Build Instructions

1. Clone this repository:
   ```powershell
   git clone https://github.com/Bink-lab/Repo-deleter.git
   cd Repo-deleter
   ```
2. Build the project using Cargo:
   ```powershell
   cargo build --release
   ```
   The compiled binary will be located at `target\release\repo-deleter.exe`.

## Usage

1. Run the program:
   ```powershell
   target\release\repo-deleter.exe
   ```
2. When prompted, enter your GitHub personal access token.
3. The tool will list your repositories. Enter the numbers (comma-separated) of the repositories you want to delete.
4. Confirm and the tool will attempt to delete the selected repositories.

### Example

```
Enter your GitHub token: <your_token>

Your repositories:
1: repo-one
2: repo-two
3: repo-three

Enter the numbers of the repositories you want to delete (comma-separated): 2,3

Deleting selected repositories...
Successfully deleted repo-two
Successfully deleted repo-three
```

## Download

Pre-built binaries (if available) can be downloaded from the [Releases page](https://github.com/Bink-lab/Repo-deleter/releases).

1. Go to the [Releases page](https://github.com/Bink-lab/Repo-deleter/releases).
2. Download the latest release for your platform (e.g., `repo-deleter.exe` for Windows).
3. Run the binary as described above.

## Security Note

**Warning:** Deleting repositories is irreversible. Use this tool with caution. Always double-check your selections before confirming deletion.

Your GitHub token is only used locally to authenticate with the GitHub API. Never share your token with others.