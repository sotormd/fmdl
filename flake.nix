{
  description = "Manage a local library based on LastFM top tracks.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs = { self, nixpkgs, naersk, ... }@inputs:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      naerskLib = pkgs.callPackage naersk { };
      latestYtDlp = pkgs.python3Packages.buildPythonApplication rec {
        pname = "yt-dlp";
        version = "2025.10.22";

        src = pkgs.fetchFromGitHub {
          owner = "yt-dlp";
          repo = "yt-dlp";
          tag = version;
          hash = "sha256-jQaENEflaF9HzY/EiMXIHgUehAJ3nnDT9IbaN6bDcac=";
        };

        pyproject = true;
        buildSystem = [ pkgs.python3Packages.hatchling ];

        nativeBuildInputs = [ pkgs.installShellFiles pkgs.pandoc ];

        propagatedBuildInputs = with pkgs.python3Packages; [
          brotli
          certifi
          mutagen
          pycryptodomex
          requests
          urllib3
          websockets
          hatchling
        ];

        doCheck = false;

        postBuild = ''
          python devscripts/prepare_manpage.py yt-dlp.1.temp.md
          pandoc -s -f markdown-smart -t man yt-dlp.1.temp.md -o yt-dlp.1
          rm yt-dlp.1.temp.md
        '';

        postInstall = ''
          install -Dm644 README.md -t $out/share/doc/yt_dlp
        '';
      };
    in {

      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          cargo
          rustc
          rustfmt
          rust-analyzer
          gcc
          openssl
          ffmpeg
          latestYtDlp
        ];
        nativeBuildInputs = [ pkgs.pkg-config ];
        env.RUST_SRC_PATH =
          "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

      packages.x86_64-linux.default = naerskLib.buildPackage {
        src = ./.;
        buildInputs = [ pkgs.gcc pkgs.openssl pkgs.ffmpeg latestYtDlp ];
        nativeBuildInputs = [ pkgs.pkg-config pkgs.makeWrapper ];
        postInstall = ''
                    wrapProgram $out/bin/fmdl \
          	    --prefix PATH : "${latestYtDlp}/bin:${pkgs.ffmpeg}/bin"
        '';
      };

      apps.x86_64-linux.default = {
        type = "app";
        program = "${self.packages.x86_64-linux.default}/bin/fmdl";
      };
    };
}
