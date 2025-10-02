### todo

1. switch to openrouter.
2. add search, so the user can just search through the models.


--- just thinking.

1. using ai + git bisect to find out which commit introduced the bug.   ***
> like first let user type the bug or like explain what's happening and then using git bisect go through the commits and find which on introduced it and also explain how one can solve it. or solve it for them


deadline : today.


keep the commit generation and the current features as options.



what's the most simple way i can build this or refactor this. 
1. using openrouter-rs for models through openrouter. https://github.com/realmorrisliu/openrouter-rs
2. keep a models.rs file to store all the models.
3. simple logic to just send the diff with system prompt to get answer.

------------- KEEP IN MIND ---------------

update the version before publishing