<b>status</b> : currently in progress.

_generate meaningful commit messages locally using AI_

acm (Auto Commit Messages) is a command-line tool that automatically generates descriptive commit messages by analyzing your git diff. It leverages the AI to provide intelligent and context-aware commit message suggestions.

<br>

<b><u>requirements</u></b>

1. you should have git installed initialised in the repo.
2. stage the changes you want to commit.
3. just run `acm run` (currently in progress) .

> if outputs the commit msg in the terminal if you like it > copy it > use it as commit msg.

<br>

### todo

- [x] get diff.

- [x] get commit msg from api.

- [ ] make it a cli tool.

- [ ] better error handling.

- [ ] supports openai, anthropic as well.

- [ ] github release.

- [ ] publish to brew.

### local development setup

> ensure `rust` is installed on your system . go to [`https://doc.rust-lang.org/book/ch01-01-installation.html`](https://doc.rust-lang.org/book/ch01-01-installation.html) for details.

1. ```bash
    git clone https://github.com/shivamhwp/acm.git
   ```
2. get an gemini-api key (currenlty only this supported) [here](https://ai.google.dev/gemini-api/docs/quickstart?lang=rest).
3. create `.env` file at root > set `GEMINI_API_KEY=" " `.
4. ```rust
    cargo build
   ```
5. `cargo run` to get the commit msg in the terminal.
6. start with `main.rs` and break stuff.
