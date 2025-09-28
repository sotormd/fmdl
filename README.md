# fmdl

Manage a local library based on LastFM top tracks.

Automatically downloads new tracks, updates metadata, and keeps your library in sync.

# Requirements

This app is packaged using [Nix](https://nixos.org/download/).

# Usage

1. Clone this repository.

    ```
    $ git clone https://github.com/sotormd/fmdl
    $ cd fmdl
    ```

2. Set up your LastFM API keys.

    Create a LastFM API account [here](https://www.last.fm/api/account/create) and create an API key.

    Then, create a `.env` file.
    ```
    $ vi .env
    ```

    Example `.env` file.
    ```
    LASTFM_API_KEY=aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    LASTFM_USERNAME=my_username
    ```

3. Run the app to sync your library.
    ```
    $ nix run .
    ```

# CLI Usage

Run with additional arguments.

`$ nix run . -- [ARGS]`

Arguments:

`-s / --sync`

Sync local library with top tracks. Default behavior.

`-d / --diff`

Show diff between local library and top tracks.

`-k / --keep-all`

Do not remove tracks marked for removal while syncing.

`-q / --shut-up`

Do not print more than necessary.

`-l / --library-path <path>`

Path to save tracks. Default: `./music`.

`-c / --cache-path <path>`

Path to download tracks to. Default: `./.cache`.

# TODO

- More metadata beyond just title and artist (album, cover art, release date, track number, ...).
- Parallel downloads.

