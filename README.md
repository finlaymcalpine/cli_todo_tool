## CLI Tool for a ToDo List

Looking to build a CLI tool that is able to do the following:
- Add, remove, edit todo items
- Mark todo items as "done"
- Save and load todo items

Thus, we need to be able to interact with items in multiple ways, as well as persist them between sessions - i.e. store both the todo item and its status (done/not done) outside of a single instance of the CLI.
For now, a text file will work for that purpose, but will migrate to a database later.
Based on a project in [[https://zerotomastery.io/blog/rust-practice-projects/]].

Possible crates to use:
- structopt
- clap
- console
- dialoguer
- tui
- rusqlite
