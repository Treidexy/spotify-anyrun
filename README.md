# Spotify

Search and play songs through Spotify.

## Installation

```sh
git clone https://github.com/Treidexy/spotify-anyrun
cd spotfiy-anyrun
cargo build --release
cp target/release/libspotify.so ~/.config/anyrun/plugins/
```

Then run with the command below, or add to your anyrun config.
```sh
anyrun --plugins libspotify.so
```

## Usage

Type in the song name.

## Configuration

```ron
// <Anyrun config dir>/spotify.ron
Config(
  max_entries: 4,
  offset: 0,
)
```