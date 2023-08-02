# hidepng

CLI tool to hide text in PNG files, written in Rust 🦀.

## Install 

- clone this repo.
- `cd` to this repo and `cargo install --path .`
- Voila!! CLI is now installed!

## Usage
- This command comes with 3 different subcommands:
  -   `encode`
  -   `decode`
  -   `remove`
- Each subcommand takes its own arguments.

## Examples

To encode: 
```
hidepng encode -f ./totallyNormalCatPhoto.png -c STXT -m "sussy msg"
```

To decode: 
```
hidpeng print -f ./totallyNormalCatPhoto.png -c STXT
```

To remove: 

```
hidepng remove -f ./totallyNormalCatPhoto.png
```
