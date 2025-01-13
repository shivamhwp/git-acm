# git-acm - git auto-commit-message

![Crates.io Total Downloads](https://img.shields.io/crates/d/git-acm)

_instant meaningful git commit messages, locally with AI_.

written in rust btw.

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

```

4. run `git-acm`.

### 📍 commands available

`use` : choose which api to run. options : `openai` | `anthropic` | `gemini`

autosaves the one you chose in `~/.config/git-acm/model.txt` file.

→ to change just run the `use` command with the preferred api.

# Contributing

### local development setup

> ensure `rust` is installed on your system . go to [`https://doc.rust-lang.org/book/ch01-01-installation.html`](https://doc.rust-lang.org/book/ch01-01-installation.html) for details.

1. ```bash
   git clone https://github.com/shivamhwp/acm.git
   ```

2. get [gemini-api-credentials](https://aistudio.google.com/app/apikey) or [openai-api-credentials](https://platform.openai.com/api-keys) or [anthropic-api-credentials](https://console.anthropic.com/settings/keys). see [.env.example](https://github.com/shivamhwp/git-acm/blob/main/.env.example)
3. create `.env` file at root > set `GEMINI_API_KEY=" " `.
4. ```bash
    cargo build
   ```
5. `cargo run --` to get the commit msg in the terminal.
6. start with `main.rs` and break stuff.

<br>

[report 🐞bugs here](https://x.com/shivamhwp)

bhai(s) : [sargam](https://x.com/sargampoudel) (idea) and [smr](https://x.com/smrdotgg) (suggestions).

<br>
