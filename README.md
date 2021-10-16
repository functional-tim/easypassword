# easypassword

This password generator was inspired by this comic of Randall Munroe: https://xkcd.com/936/.

It is cryptographically secure if the machine you are using it on is secure.

## How to use

```
easypassword [OPTIONS] file seperator1 seperator2
```

The file has to have one word per line.

To make the passwords more secure two seperators are choosen.

These seperators will alternate after every word. One seperator should be a special character and the other seperator should be a number.

## How to install

### Using cargo
You need to install cargo on your system through your package manager or any other means.

Then you simply install it through cargo.

```
$ > cargo install easypassword
```

### Using source
You need to install cargo on your system through your package manager or any other means.

Then  you download the repository through git or manual.

After unpacking or downloading from git you have to switch into the folder of easy-password generator.

Then run `cargo install --path .`.


```
$ > cd easypassword
$ > cargo install --path
```

## Credits

- Randall Munroe for the idea
