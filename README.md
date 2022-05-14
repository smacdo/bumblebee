# Spellingbee
![CI build state](https://img.shields.io/github/workflow/status/smacdo/spellingbee/Rust/main)
![code license](https://img.shields.io/github/license/smacdo/spellingbee)
![Crate version](https://img.shields.io/crates/v/spellingbee)

Spellingbee is a command line program (and reusable library) for discovering 
answers to the daily NYT spelling bee challenge. It also shows each answer's
score and highlights any pangram answers.

## Installation
Use Rust's [cargo](https://www.rust-lang.org/tools/install) tool to install Spellingbee. 

```shell
$ cargo install spellingbee
$ spellingbee o cbiprt     # if $PATH contains `~/.cargo/bin`
```

Please contact me or create a new issue if you would like to have downloadable
installers for your platform rather than use cargo.

## Usage
### Command line
Spellingbee can be invoked on the command line once it is installed. Simply 
pass the required letter as the first argument, and then all of the other
letters as the second argument.

You can also use the `-d path/to/dictionary` if you would like to use an
alternative word list. The default world list uses your operating system's
dictionary which contains many more words than the NYT spelling bee game will
accept.

For additional information on using the command line tool invoke the tool like
this: `spellingbee --help`.
### Library
Spellingbee is also available as a reusable Rust library. The latest API
documentation is available from a link on [spellingbee's crates.io page](https://crates.io/crates/spellingbee).

Generally all you need is the `check_word` function, as seen here in this 
example:

```rust
use spellingbee::check_word;
assert!(check_word("loon", 'o', "unrlap").is_some());
```

## Building
Make sure you install [Rust](https://www.rust-lang.org/tools/install) on your
computer before building.

```shell
$ git clone git@github.com:smacdo/spellingbee.git
$ cd spellingbee
$ cargo test
$ cargo run -- o cbiprt
```


## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.