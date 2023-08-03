# hidepng

CLI tool to hide text in PNG files, written in Rust 🦀.

## Install 

- clone this repo.
- `cd` to this repo and `cargo install --path .`

## Usage
- This command comes with 3 different subcommands:
  -   `encode`
  -   `decode`
  -   `remove`
- Each subcommand takes its own arguments.
- `--help` arg can be supplied with every subcommand and the main command itself.

## Examples

To encode: 
```
hidepng encode -f ./totallyNormalCatPhoto.png -m "sussy msg"
```

To decode: 
```
hidpeng print -f ./totallyNormalCatPhoto.png 
```

To remove: 

```
hidepng remove -f ./totallyNormalCatPhoto.png
```
