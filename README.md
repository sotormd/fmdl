# fmdl

Manage a local library based on LastFM top tracks.

Automatically downloads new tracks, updates metadata, and keeps your library in sync.

# Requirements

This app is packaged using [Nix](https://nixos.org/download/) for `x86_64-linux` targets.

A LastFM API key is required. Create a LastFM API account [here](https://www.last.fm/api/account/create) and get an API key.

# Usage

```console
$ nix run github:sotormd/fmdl
```
> Make sure you set the environment variables `LASTFM_API_KEY` and `LASTFM_USERNAME` either via the console or via a `.env` file in the current working directory.

# CLI Usage

Run with additional arguments.

`$ nix run github:sotormd/fmdl -- [ARGS]`

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

Path to download tracks. Default: `./.cache`.

# Features

- [x] Sync local library with LastFM top tracks
- [x] Show diff between local library and top tracks
- [x] Basic metadata for saved `.mp3` files
- [x] Basic cli with `clap`
- [ ] Rich metadata (album, cover art, release date, track number, ...)
- [ ] Parallel downloads

