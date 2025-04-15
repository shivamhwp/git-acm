# git-acm - git auto-commit-message

### instant meaningful commit messages

![Crates.io Total Downloads](https://img.shields.io/crates/d/git-acm?labelColor=%23222&color=white)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/shivamhwp/git-acm/release.yml?labelColor=%23222&color=white)

### installation

```
curl -sSL https://raw.githubusercontent.com/shivamhwp/git-acm/main/install.sh | sh
```

or
if you have `cargo` installed on your system.

```
cargo install git-acm
```

### how to use

1. add [gemini-api-key](https://aistudio.google.com/app/apikey) or [openai-api-key](https://platform.openai.com/api-keys) or [anthropic-api-key](https://console.anthropic.com/settings/keys) or [deepseek-api-key](https://platform.deepseek.com/api_keys)
2. add these in your project's `.env` file or `export` them in terminal.

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

4. run `git-acm`.

### 📍 commands available

`use` : choose which model you want to use. (run `git-acm list` to see the available models).

`autocommit` : enables or disables the autocommit functionality.

`list` : lists all available models.

### 📍 example

```bash
git-acm use <model_name>     # choose which model to use.
git-acm list 				 # list all the models.
git-acm autocommit enable    # Enable automatic commits with generated messages
git-acm autocommit disable   # Disable automatic commits
git-acm                      # Generate a commit message using the currently selected API

```
