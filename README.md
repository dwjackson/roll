# Roll: Roll Virtual Dice

Roll is a simple command-line utility that can be used to simulate rolling a
number of dice of various sizes.

## Usage

Roll parses strings of the form `#d#` where the first `#` corresponds to the
number of dice to roll and the second `#` corresponds to the number of sides
on the die. For example:

```sh
$ roll 2d6 3d8
```

will roll two six-sided dice and three eight-sided dice. Roll will print
results one per line and then a "total" line at the end.

## License

Roll is licensed under the MIT License.
