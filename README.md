# CS2shock

## How it works

Simple, you get shocked with a duration of 1 and a intensity of `max_shock_intensity` (configurable) when you die during a live match (so not warmup).

Matches do not need to be premier or comp, can be any match.

There are also two options to beep whenever a match starts, and whenever a round starts. For you forgetful folks.

## Usage

1. Find you install directiory, do to so go to steam, right click on Counter-Strike 2, go to `Manage > Browse Local Files`.
2. Put `gamestate_integration_cs2shock.cfg` in the `game/csgo/cfg` folder. (NOT THE `csgo/cfg` folder). You can now close this folder.
3. Place `cs2shock.exe` anywhere, with `config.toml` next to it in the same folder. Fill out the values in `config.toml`.
4. You can now run `cs2shock.exe`.

## Building from source

### Prerequisites

-   [Rust installed](https://doc.rust-lang.org/cargo/getting-started/installation.html)

1. Clone the repository
    - `git clone https://github.com/VolcanoCookies/cs2shock.git`
2. Open the created folder
    - `cd cs2shock`
3. Build the project
    - `cargo build --release`

You can then find the executable in `cs2shock/target/release/cs2shock.exe`
