# Website blocker

This program allows you to block and unblock websites from the command line by modifying the hosts file.
<br>
## Installation

**You can use the program by downloading a binary from the [releases](https://github.com/bython17/website_blocker/releases/tag/0.1.0)**. For manual builds use the following method.
<br>
<br>
**NOTE**: Make sure to have the rust compiler and `cargo` installed.

- Clone the repository and `cd` into it
```bash
git clone https://github.com/bython17/website_blocker && cd website_blocker
```
- Then build the app in release mode
```bash
cargo build --release
```

## Usage
Using the app is very simple
- To list the currently blocked sites
```bash
website_blocker list
```
- To block a site using the app
```bash
website_blocker block anyste.com 0.0.0.0
```
- To unblock a site using the app
```bash
website_blocker unblock anysite.com
```
**NOTE**: You will need elevated privileges to run the program. That means using `cmd` or `powershell` in administrative mode in windows
or by using `sudo` before the command in *nix systems.

### Notice
This is a program that I wrote to learn the `rust` programming language. It has a very basic code and it might not be efficient at all. There are
a lot of better programs than this one that allow blocking sites in a more better way. So I will only recommend this for reading the code.
