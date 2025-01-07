# git-acm - git auto-commit-message

_generate meaningful commit messages locally using AI_.

### ❗requirements</u></b>

1. you should have git installed initialised in the repo.
2. stage the changes you want to commit.
3. just run `git-acm` or `git-acm run`.

### how to use

> ensure `rust` is [`installed`](https://doc.rust-lang.org/book/ch01-01-installation.html) on your system and requirements are met.

1. run `cargo install git-acm`.
2. add [gemini-api-key](https://aistudio.google.com/app/apikey).
3. add this ⬇️︎ in your project's `.env` file.

```JSON
   GEMINI_API_KEY="YOUR_API_KEY"
   GEMINI_API_URL="https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent"
```

4. run `git-acm`.

 <br>
