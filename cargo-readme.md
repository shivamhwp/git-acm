# git-acm - git auto-commit-message

### instant meaningful commit messages powered by OpenRouter

[![Docs](https://img.shields.io/badge/docs-git--acm.pages.dev-blue?logo=readthedocs)](https://git-acm.pages.dev) [![Crates.io](https://img.shields.io/badge/crates.io-git--acm-orange?logo=rust)](https://crates.io/crates/git-acm)

![Crates.io Total Downloads](https://img.shields.io/crates/d/git-acm?labelColor=%23222&color=white)

### installation

```
curl -sSL https://raw.githubusercontent.com/shivamhwp/git-acm/main/install.sh | sh
```
### how to use

1. Ensure git is initialized (`git init` if needed).
2. Get [OpenRouter API key](https://openrouter.ai/keys) and add to `.env` or export:
   ```
   OPENROUTER_API_KEY="your_key"
   ```
3. Run `git-acm get-models` to fetch models (run once, stores locally). Use `git-acm list` to view.
4. Visit [openrouter.ai/models](https://openrouter.ai/models), copy model ID.
5. Run `git-acm use <copied_model_id>` to select (e.g., `anthropic/claude-sonnet-4.5`).
6. Stage changes: `git add .`
7. Run `git-acm` to generate and use the commit message (copies to clipboard automatically).

### 📍 commands available
`get-models` : Fetch models from OpenRouter.
`use <model>` : Select model.
`list` : List models.
`autocommit enable/disable` : Toggle auto-commit.


### 📍 example

```bash
git-acm get-models
git-acm list
git-acm use anthropic/claude-sonnet-4.5
git-acm autocommit enable
git-acm
```
<br/>
> **config storage** <br/>
>`git-acm-prefs.json` (model + autocommit prefs.) and `models.json` (list of models available) are stored in your OS config directory.  
> - **macOS:** `~/Library/Application Support/git-acm`  
> - **Linux:** `$XDG_CONFIG_HOME/git-acm`  
> - **Windows:** `%APPDATA%/git-acm`
