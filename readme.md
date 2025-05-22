# git-acm ( git auto-commit-message )

### instant meaningful commit messages.

![Crates.io Total Downloads](https://img.shields.io/crates/d/git-acm?labelColor=%23222&color=white)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/shivamhwp/git-acm/release.yml?labelColor=%23222&color=white)

docs 📄 : [git-acm.pages.dev](https://git-acm.pages.dev) | crate 🦀 : [crates.io](https://crates.io/crates/git-acm)

### installation

```
curl -sSL https://raw.githubusercontent.com/shivamhwp/git-acm/main/install.sh | sh
```

> to update : just run the installation command, it checks the system for prev version and then installs a new version if there's one.

or
if you have `cargo` installed on your system.

```
cargo install git-acm
```

### how to use

1. ensure git is initialized in the dir. or run `git init`.
2. add [gemini-api-key](https://aistudio.google.com/app/apikey) or [openai-api-key](https://platform.openai.com/api-keys) or [anthropic-api-key](https://console.anthropic.com/settings/keys) or [deepseek-api-key](https://platform.deepseek.com/api_keys)
3. add these in your project's `.env` file (preferred) or `export` them in terminal.

```
# for gemini api

GEMINI_API_KEY=""

# for anthropic api

ANTHROPIC_API_KEY=""

# for openai api

OPENAI_API_KEY=""

# for llama api (using ollama)

LLAMA_API_URL="http://localhost:11434/api/generate"
LLAMA_MODEL_NAME= "llama3.2:1b"

# for deepseek api

DEEPSEEEK_API_KEY=""

```

4. run `git-acm use <model_name>`.
5. just run `git-acm`.

### 📍 commands available

`use` : choose which model you want to use. (run `git-acm list` to see the available models).

`autocommit` : enables or disables the autocommit functionality.

`list` : lists all available models.

### 📍 example

```bash
git-acm use <model_name>     # choose which model to use.
git-acm autocommit enable    # Enable automatic commits with generated messages
git-acm list                 # lists all the available models.
git-acm autocommit disable   # Disable automatic commits
git-acm                      # Generate a commit message using the currently selected model.

```

# Contributing

### local development setup

> ensure `rust` is installed on your system . go to [`https://doc.rust-lang.org/book/ch01-01-installation.html`](https://doc.rust-lang.org/book/ch01-01-installation.html) for details.

1. ```bash
   git clone https://github.com/shivamhwp/acm.git
   ```

2. get [gemini-api-credentials](https://aistudio.google.com/app/apikey) or [openai-api-credentials](https://platform.openai.com/api-keys) or [anthropic-api-credentials](https://console.anthropic.com/settings/keys). see [.env.example](https://github.com/shivamhwp/git-acm/blob/main/.env.example)
3. create `.env` file at root > set `GEMINI_API_KEY=" " ` or any other provider's key.
4. ```bash
    cargo build
   ```
5. `cargo run --` to get the commit msg in the terminal.
6. start with `main.rs` and break stuff.

<br>

[report 🐞bugs here](https://x.com/shivamhwp)

bhai(s) : [sargam](https://x.com/sargampoudel) (idea) and [smr](https://x.com/smrdotgg) (suggestions).

<br>
