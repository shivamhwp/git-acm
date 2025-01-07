_generate meaningful commit messages locally using AI_

acm (Auto Commit Messages) is a command-line tool that automatically generates descriptive commit messages by analyzing your git diff. It leverages the AI to provide intelligent and context-aware commit message suggestions.

> some features are currently in progress. it's works with gemini-api rn. and available as pkg for your rust project. working on the binary release.

<br>

> ‼ keep in mind, this requirements must be met, i'm working on error handling so these things don't cause much problems. ty.

<b><u>requirements</u></b>

1. you should have git installed initialised in the repo.
2. stage the changes you want to commit.
3. just run `acm` or `acm run`.

> if outputs the commit msg in the terminal if you like it > copy it > use it as commit msg.

## todo

- [x] get diff.
- [x] get commit msg from api.
- [x] make it a cli tool.
- [ ] publish to cargo as a pacakge.
- [ ] better error handling.
- [ ] supports openai, anthropic as well.
- [ ] github release.
- [ ] publish to brew or binary release.

maybe later -> a website for this project.

## local development setup

> ensure `rust` is installed on your system . go to [`https://doc.rust-lang.org/book/ch01-01-installation.html`](https://doc.rust-lang.org/book/ch01-01-installation.html) for details.

1. ```bash
    git clone https://github.com/shivamhwp/acm.git
   ```
2. get an gemini-api key (currenlty only this supported) [here](https://ai.google.dev/gemini-api/docs/quickstart?lang=rest).
3. create `.env` file at root > set `GEMINI_API_KEY=" " `.
4. ```bash
    cargo build
   ```
5. `cargo run` to get the commit msg in the terminal.
6. start with `main.rs` and break stuff.

<br>

> ### give your suggestions on what features i should add.

<br>

me : [shivam.ing](https://shivam.ing)

<br>

bhai : [sargam](https://x.com/sargampoudel) (project idea)
