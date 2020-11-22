# Roll: Roll Virtual Dice

Roll is a simple command-line utility that can be used to simulate rolling a
number of dice of various sizes.

## Usage

Roll parses strings of the form `#d#` where the first `#` corresponds to the
number of dice to roll and the second `#` corresponds to the number of sides
on the die. This is referred to as
[dice notation](https://en.wikipedia.org/wiki/Dice_notation). For example:

```sh
$ roll 2d6 3d8
```

will roll two six-sided dice and three eight-sided dice. Roll will print
results one per line and then a "total" line at the end.

A "fudge-die" (2 sides with a "+", 2 sides with a "-", and 2 blank sides) can
be rolled as a `dF` or `df`.

## Custom Dice

To roll a custom die, the "shape" of the die must be given and the result
must be a number. For example, to roll a Fate-Core-style die without using the
`dF` notation:

```sh
roll '1d{1,1,0,0,-1,-1}'
```

## License

Roll is licensed under the MIT License.
