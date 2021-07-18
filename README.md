# easy-password-generator

This password generator was inspired by this comic of Randall Munroe: https://xkcd.com/936/.

It is cryptographically secure if the machine you are using it on is secure.

## How to use

```
easy-password [OPTIONS] file seperator1 seperator2
```

The file has to have one word per line.

To make the passwords more secure two seperators are choosen.

These seperators will alternate after every word. One seperator should be a special character and the other seperator should be a number.

### Example uses

```
$ > easy-password example_word_files/word_list.txt % 5
Fewer%Trees5Cleaning%Kitty5
$ > easy-password --number=5 example_word_files/word_list.txt '$' 8
Symbol$Teacher8Requieres$Minimal8Pills$
```

## How to install

### Using cabal
You need to download cabal or install it on your system trhough your package manager.

Then  you download the repository through git or manual.

After unpacking or downloading from git you have to switch into the folder of easy-password generator.

Then run `cabal new-install`.


```
$ > cd easy-password-generator
$ > cabal new-install
```

### Using stack
You need to download stack or install it on your system through your package manager.

Then you download the repository through git or manual.

After unpacking or downloading from git you have to switch into the folder of easy-password-generator.

Then run `stack setup` followed by `stack install`.

```
$ > cd easy-password-generator
$ > stack setup
$ > stack install
```

### Using nix
You need to download nix or install it on your system trhough your package manager.

Then  you download the repository through git or manual.

After unpacking or downloading from git you have to switch into the folder of easy-password generator.

Then run `nix-build release.nix`.
```
$ > cd easy-password-generator
$ > nix-build release.nix
```

## Credits

- Randall Munroe for the idea
- Piyush P Kurur for the cryptographic library [raaz](https://hackage.haskell.org/package/raaz-0.2.1)
- first20hours for the [word lists](https://github.com/first20hours/google-10000-english)
