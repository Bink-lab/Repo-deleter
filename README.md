# Repo Deleter

Repo Deleter is a command-line tool written in Rust that allows you to list and delete your GitHub repositories in bulk. It uses the GitHub API and requires a personal access token with the appropriate permissions.

**⚠️ IMPORTANT:** This tool permanently deletes repositories. Always double-check your selections and consider using `--dry-run` first.

## Features

- **Safe by default**: Filters out forks and archived repositories by default
- **Dry-run mode**: Preview what would be deleted without actually deleting anything
- **Interactive confirmation**: Requires typing "DELETE" to confirm deletions (unless `--yes` is used)
- **Flexible token input**: Accepts token via CLI argument, environment variable, or interactive prompt
- **Pagination support**: Handles GitHub API pagination to list all repositories
- **Enhanced display**: Shows repository visibility (public/private) and tags (fork/archived)
- **Range selection**: Supports comma-separated indices and ranges (e.g., `1,3-5,7`)
- **Concurrent deletion**: Configurable concurrency to speed up bulk operations
- **Robust error handling**: Clear error messages and graceful failure handling

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- A GitHub personal access token with `repo` and `delete_repo` permissions

## Build Instructions

1. Clone this repository:
   ```bash
   git clone https://github.com/Bink-lab/Repo-deleter.git
   cd Repo-deleter
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```
   The compiled binary will be located at `target/release/repo-deleter` (or `target/release/repo-deleter.exe` on Windows).

## Usage

### Basic Usage

```bash
# Interactive mode (prompts for token)
./target/release/repo-deleter

# With token from environment variable
export GITHUB_TOKEN="your_token_here"
./target/release/repo-deleter

# With token via command line
./target/release/repo-deleter --token "your_token_here"
```

### Safety Features

```bash
# Dry run - see what would be deleted without actually deleting
./target/release/repo-deleter --dry-run

# Non-interactive mode (skips "DELETE" confirmation)
./target/release/repo-deleter --yes

# Include forks and archived repositories
./target/release/repo-deleter --include-forks --include-archived
```

### Advanced Options

```bash
# Adjust concurrency for faster deletion
./target/release/repo-deleter --concurrency 8

# Adjust page size for API requests
./target/release/repo-deleter --per-page 50

# Combine options
./target/release/repo-deleter --dry-run --include-forks --concurrency 2
```

### Command-Line Options

| Option | Description |
|--------|-------------|
| `-t, --token <TOKEN>` | GitHub token (overrides environment variable) |
| `--dry-run` | Preview mode - shows what would be deleted |
| `-y, --yes` | Skip interactive confirmation |
| `--include-forks` | Include forked repositories |
| `--include-archived` | Include archived repositories |
| `--concurrency <N>` | Maximum concurrent delete requests (default: 4) |
| `--per-page <N>` | API page size, max 100 (default: 100) |
| `-h, --help` | Show help information |
| `-V, --version` | Show version information |

### Example Session

```
$ ./target/release/repo-deleter --dry-run

Your repositories:
1: user/test-repo-1 (public)
2: user/test-repo-2 (private)
3: user/forked-repo (public) [fork]
4: user/old-project (public) [archived]

Enter the numbers of the repositories you want to delete (comma-separated, ranges allowed): 1,3-4

Selected repositories to be deleted:
- user/test-repo-1
- user/forked-repo
- user/old-project

Dry-run mode enabled. No repositories will be deleted.
```

### Selection Syntax

The tool supports flexible selection syntax:
- **Single numbers**: `1` (selects repository 1)
- **Comma-separated**: `1,3,5` (selects repositories 1, 3, and 5)
- **Ranges**: `1-5` (selects repositories 1 through 5)
- **Mixed**: `1,3-5,7` (selects repositories 1, 3, 4, 5, and 7)

## Environment Variables

| Variable | Description |
|----------|-------------|
| `GITHUB_TOKEN` | GitHub personal access token (used if `--token` not provided) |

## Safety Notes

- **Repository deletion is permanent and cannot be undone**
- By default, forks and archived repositories are excluded from the list
- The tool requires typing "DELETE" (uppercase) to confirm deletion
- Use `--dry-run` to preview what would be deleted before running the actual deletion
- All operations respect GitHub API rate limits with built-in delays
- Failed deletions are reported with detailed error messages

## Token Permissions

Your GitHub personal access token must have the following permissions:
- `repo` (full repository access)
- `delete_repo` (delete repository access)

To create a token:
1. Go to GitHub Settings → Developer settings → Personal access tokens
2. Click "Generate new token"
3. Select the required permissions
4. Copy the token and use it with this tool

**Never share your GitHub token or commit it to version control.**

## Download

Pre-built binaries may be available from the [Releases page](https://github.com/Bink-lab/Repo-deleter/releases).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is open source. Please check the repository for license details.