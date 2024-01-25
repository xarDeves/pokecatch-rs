# pokecatch-rs
An expanded version of [pokeget-rs](https://github.com/talwat/pokeget-rs) with extra functionality for catching pokemons and using items.

## Usage
Configure your terminal to run `pokecatch` each time you open it or just run `pokecatch` on your terminal.

## Commands
- `pokecatch` : Trigger a pokemon encounter, along with the chance of encountering pokeballs and items.
- `pokecatch throw <ball>` : Attempt to catch the encountered pokemon.
  - Options:
    - `pokeball`
    - `greatball`
    - `ultraball`
    - `masterball`
- `pokecatch bag use <item>` : Use an item (or multiple) from your bag.
  - Options:
    - `shiny-bait`
    - `legendary-bait`
    - `legendary-bait shiny-bait`
- `pokecatch pc show --<which>` :  Display pokemon.
  - Options:
    - `--all`
    - `--caught`
    - `--uncaught`
- `pokecatch --help`


## Configuration
You can set the encounter chances and effectiveness of items and balls by altering the config files in the data/config directory

## Adding a directory to $PATH
#### Bash & Zsh
Append this to your `.bashrc` or `.zshrc`:
```sh
export PATH="<path>:$PATH"
```

#### Fish
Run this in your CLI:
```sh
fish_add_path <path>
```

## Credits
Original project: [pokeget-rs](https://github.com/talwat/pokeget-rs) by [talwat](https://github.com/talwat)

This time, the sprites are from [pokesprite](https://github.com/msikma/pokesprite) and pokecatch uses them with a submodule.

Sprites are embedded into the binary, so pokecatch won't download them.