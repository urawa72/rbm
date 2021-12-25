# rbm

A command line bookmark management app written in Rust!

## Install

### Cargo install

```bash
$ cargo install rbm
```

## Usage

```bash
$ rbm -h
```

Add bookmark with `add` subcommand.  
Bookmark is stored in `~/rbm-bookmarks.toml`.

- Title: bookmark title
- URL: bookmark URL
- Tag: bookmark tags (If there are multiple, separate them with commas)

```bash
$ rbm add
Title> Google
URL> https://google.com
Tag> Google,Search
```

List bookmarks with `list` subcommand.  
You can do an ambiguous search by title and tags.  
Select a bookmark to open it in your browser (Only for macOS function).

```bash
$ rbm list
```

You can change the location of `rbm-bookmarks.toml` by changing the path of the `~/rbm-config.toml`.

```toml
[config]
path = '/Users/urawa72/rbm-bookmarks.toml'
```
