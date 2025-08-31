# repo-deleter

A Rust-based command-line tool to delete GitHub repositories.

## Building the Project

To build the `repo-deleter` executable, you need to have Rust and Cargo installed. If you don't have them, you can install them from [rust-lang.org](https://www.rust-lang.org/tools/install).

1.  **Clone the repository:**
    ```bash
    git clone <repository-url>
    cd repo-deleter
    ```
2.  **Build in release mode (optimized for size):**
    ```bash
    cargo build --release
    ```
    The executable will be located at `target/release/repo-deleter.exe` (on Windows) or `target/release/repo-deleter` (on Linux/macOS).

## GitHub Personal Access Token (PAT)

This tool requires a GitHub Personal Access Token (PAT) to authenticate with the GitHub API and delete repositories.

### How to Obtain a GitHub PAT:

1.  Go to your GitHub settings: [https://github.com/settings/tokens](https://github.com/settings/tokens)
2.  Click on "Generate new token" or "Generate new token (classic)". For this tool, a "classic" token is sufficient.
3.  **Note:** GitHub recommends using fine-grained tokens for better security. If you choose a fine-grained token, ensure it has the necessary permissions to delete repositories. For a classic token, you will need to grant the `delete_repo` scope.
4.  Give your token a descriptive name (e.g., "repo-deleter-cli").
5.  **Crucially, copy the generated token immediately.** You will not be able to see it again once you leave the page.

### Using the PAT with `repo-deleter`:

The `repo-deleter` tool will likely expect the PAT as an environment variable or a command-line argument. Please refer to the tool's usage instructions for the exact method. A common approach is to set an environment variable like `GITHUB_TOKEN`:

**Windows (Command Prompt):**
```cmd
set GITHUB_TOKEN=YOUR_PERSONAL_ACCESS_TOKEN
repo-deleter.exe <arguments>
```

**Windows (PowerShell):**
```powershell
$env:GITHUB_TOKEN="YOUR_PERSONAL_ACCESS_TOKEN"
.epo-deleter.exe <arguments>
```

**Linux/macOS:**
```bash
export GITHUB_TOKEN="YOUR_PERSONAL_ACCESS_TOKEN"
./repo-deleter <arguments>
```

**Replace `YOUR_PERSONAL_ACCESS_TOKEN` with the actual token you generated.**
