# git-acm - git auto-commit-message

![Crates.io Total Downloads](https://img.shields.io/crates/d/git-acm) ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/shivamhwp/git-acm/release.yml)

_generate meaningful commit messages locally using AI_.

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

1. add [gemini-api-key](https://aistudio.google.com/app/apikey) or [openai-api-key](https://platform.openai.com/api-keys) or [anthropic-api-key](https://console.anthropic.com/settings/keys)
2. add these in your project's `.env` file or `export` them in terminal.

```
# for gemini api

GEMINI_API_URL="https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent"
GEMINI_API_KEY=""

# for anthropic api

ANTHROPIC_API_URL="https://api.anthropic.com/v1/messages"
ANTHROPIC_API_KEY=""

# for openai api

OPENAI_API_URL="https://api.openai.com/v1/chat/completions"
OPENAI_API_KEY=""

# for llama api (using ollama)

LLAMA_API_URL="http://localhost:11434/api/generate"
LLAMA_MODEL_NAME= "llama3.2:1b"

```

4. run `git-acm`.

### 📍 commands available

`use` : choose which api to run. options : `openai` | `anthropic` | `gemini` | `llama` .

`autocommit` : enables or disables the autocommit functionality.

### 📍 example

```bash
    git-acm use <api_name>       # Choose which API to use (openai, anthropic, gemini, llama)
    git-acm autocommit enable    # Enable automatic commits with generated messages
    git-acm autocommit disable   # Disable automatic commits
    git-acm                      # Generate a commit message using the currently selected API

```
