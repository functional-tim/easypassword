# easypassword

[![Crates.io](https://img.shields.io/crates/v/easypassword.svg)](https://crates.io/crates/easypassword)
[![License](http://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/functional-tim/LICENSE-MIT)
[![dependency status](https://deps.rs/repo/github/functional-tim/easypassword/status.svg)](https://deps.rs/repo/github/functional-tim/easypassword)

-----------------------------------------------

This password generator was inspired by this comic of Randall Munroe: https://xkcd.com/936/.

It is cryptographically secure if the machine you are using it on is secure.

## How to use

```
easypassword [OPTIONS] seperator1 seperator2
```

To make the passwords more secure two seperators are choosen.

These seperators will alternate after every word. One seperator should be a special character and the other seperator should be a number. You can choose which one is which.

If you want to use your own word list file you can do so through the option `-i` or `--input`.
Each line of the file should be a single word or an open compound word.

```
easypassword -i file seperator1 seperator2
```

## How to install

### Using cargo
You need to install cargo on your system through your package manager or any other means.

Then you simply install it through cargo.

```
$ > cargo install easypassword
```

### Using nix
You need to install nix on your system through your package manager or any other means.

After unpacking or downloading from git you have to switch into the folder of easypassword.

Then you simply run the following commands.

```
$ > nix-build
$ > nix-env -i ./result
```

### Using source
You need to install cargo on your system through your package manager or any other means.

Then  you download the repository through git or manual.

After unpacking or downloading from git you have to switch into the folder of easypassword.

Then run `cargo install --path .`.

```
$ > cd easypassword
$ > cargo install --path
```

## License
easypassword is dual licensed under [MIT License](LICENSE-MIT) and [Apache 2 License](LICENSE-APACHE).

All data in the directory of 12dicts are under the following license:
> The 12dicts lists were compiled by Alan Beale. I explicitly release them to the public domain, but request acknowledgment of their use. (Actually, the dependency of the 2of12inf list and the 2+2+3 lists on AGID prevents their release into the public domain. However, I do not impose any additional requirements on their use beyond those imposed by AGID and its sources, as described in [agid.txt](12dicts/agid.txt).)

## Credits and Acknowledgments

- Randall Munroe for the idea
- Alan Beale for 12dicts word lists (http://wordlist.aspell.net/12dicts/)
