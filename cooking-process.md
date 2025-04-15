# what i think rn will be the process of building it.

this file is just my documentaion of thoughts, when i was building this.

### what are the main things we need this tool to work :

- i think git diffs. so you can get the diff b/w the previous and current commit.

- get the files changed > send the git diff in the api call > get the res back > show it in the terminal (maybe).

<br/>

### just thinking

- sending the full git diff is not feasible, coz of the token limit. so the second best thing we can do this is either sending files in patches or limit them based on lines.(maybe in future)

> in case of patches, get the commit messages > store them in a json file with their respec. file names and them make the last api call to get the commit msg for the pr. (maybe in future)

- if the change is not that big, then we git diff > req to api > res from api > set it as gcm.

<br/>

### things to consider

- removing the empty lines from git diff.

<br/>

### what will be the process (as of think rn):

- you install the binary or the package. (it gets stored in .acm dir in your root dir. which contains an executable whenever you run acm command in terminal)

- i think that's it. let's build this.

---

so solving the msg choosing and running feature, so if we think what could be the flow.
user gets the msg from the response > [enter] to choose and [r] to rerun and get another msg.

so a fn that takes a string as input and watches for keycodes, based on which either it calls the function or returns the result.

todo

[ ] revamp the structure and let user directly put model names and we figure out the rest.
