# git-acm

**Instant commit messages in the terminal** powered by [OpenRouter](https://openrouter.ai/)

[![Docs](https://img.shields.io/badge/docs-git--acm.pages.dev-blue?logo=readthedocs)](https://git-acm.pages.dev) [![Crates.io](https://img.shields.io/badge/crates.io-git--acm-orange?logo=rust)](https://crates.io/crates/git-acm)

![Crates.io Total Downloads](https://img.shields.io/crates/d/git-acm?labelColor=%23222&color=white)

### Features
- Generates concise, meaningful commit messages from staged changes.
- Access to mostly all models through [OpenRouter](https://openrouter.ai/).
- Copies message to clipboard.
- Auto-commit option.
- Excludes common build dirs from diff (node_modules, target, etc.).


### installation

```
curl -sSL https://raw.githubusercontent.com/shivamhwp/git-acm/main/install.sh | sh
```

> to update : just run the installation command, it checks the system for prev version and then installs a new version if there's one.

### how to use

1. ensure git is initialized in the dir. or run `git init`.
2. get [OpenRouter API key](https://openrouter.ai/keys)
3. add it in your project's `.env` file (preferred) or `export` it in terminal.
```
OPENROUTER_API_KEY="your_api_key_here"
```
4. run `git-acm get-models` to fetch available models (run once, stores locally). you can run `git-acm list` to see the list of the models.
5. go to [openrouter.ai/models](openrouter.ai/models), click copy model id. 
![Model selection screenshot](https://ypazyw0thq.ufs.sh/f/38t7p527clgq7em4D2IYty0zsu2PpBGJxga1efWZASI7i4DU)
6. run `git-acm use <copied_model_id>` to select the model.
7. stage your changes with `git add .` or specific files.
8. just run `git-acm` to generate and use the commit message(it already gets copied to the clipboard automatically).

### 📍 commands available

`get-models` : Fetch and store models from OpenRouter (run occasionally to update).

`use <model>` : Choose a model.

`list` : Lists all available models.

`autocommit enable/disable` : Enables/disables automatic git commit after generation.

`run` : Generate a commit message (default behavior).

### 📍 example

```bash
git-acm get-models                        # Fetch models (first time)
git-acm list                              # See available models
git-acm use anthropic/claude-sonnet-4.5   # Select model
git-acm autocommit enable                 # Enable auto-commit
git-acm                                   # Generate commit message
git-acm autocommit disable                # Disable auto-commit
```

# Contributing

### local development setup

> ensure `rust` is installed on your system . go to [`https://doc.rust-lang.org/book/ch01-01-installation.html`](https://doc.rust-lang.org/book/ch01-01-installation.html) for details.

1. ```bash
   git clone https://github.com/shivamhwp/git-acm.git
   ```
2. get [OpenRouter API key](https://openrouter.ai/keys). see [.env.example](https://github.com/shivamhwp/git-acm/blob/main/.env.example)
3. create `.env` file at root > set `OPENROUTER_API_KEY="your_key"`.
4. ```bash
    cargo build
   ```
5. `cargo run --` to get the commit msg in the terminal.
6. start with `main.rs` and break stuff.

<br>

[report 🐞bugs here](https://x.com/shivamhwp)

bhai(s) : [sargam](https://x.com/sargampoudel) (idea) and [smr](https://x.com/smrdotgg) (suggestions).

<br>
