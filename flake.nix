{
  description = "Manage a local library based on LastFM top tracks.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    naersk.url = "github:nix-community/naersk";
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];

      ytDlpVersion = "2025.10.22";
      ytDlpHash = "sha256-jQaENEflaF9HzY/EiMXIHgUehAJ3nnDT9IbaN6bDcac=";

      # function to build yt-dlp for a given pkgs
      mkYtDlp =
        pkgs:
        pkgs.python3Packages.buildPythonApplication rec {
          pname = "yt-dlp";
          version = ytDlpVersion;

          src = pkgs.fetchFromGitHub {
            owner = "yt-dlp";
            repo = "yt-dlp";
            tag = version;
            hash = ytDlpHash;
          };

          pyproject = true;
          buildSystem = [ pkgs.python3Packages.hatchling ];

          nativeBuildInputs = with pkgs; [
            installShellFiles
            pandoc
          ];

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

      # generic per-system builder
      forSystem =
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          naerskLib = pkgs.callPackage naersk { };
          ytDlp = mkYtDlp pkgs;
        in
        {
          devShells.default = pkgs.mkShell {
            buildInputs =
              with pkgs;
              [
                cargo
                rustc
                rustfmt
                rust-analyzer
                gcc
                openssl
                ffmpeg
              ]
              ++ [ ytDlp ];
            nativeBuildInputs = [ pkgs.pkg-config ];
            env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
          };

          packages.default = naerskLib.buildPackage {
            src = ./.;
            buildInputs =
              with pkgs;
              [
                gcc
                openssl
                ffmpeg
              ]
              ++ [ ytDlp ];
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.makeWrapper
            ];
            postInstall = ''
              wrapProgram $out/bin/fmdl \
                --prefix PATH : "${ytDlp}/bin:${pkgs.ffmpeg}/bin"
            '';
          };

          apps.default = {
            type = "app";
            program = "${self.packages.${system}.default}/bin/fmdl";
          };
        };

    in
    {
      devShells = builtins.listToAttrs (
        map (system: {
          name = system;
          value = (forSystem system).devShells;
        }) systems
      );

      packages = builtins.listToAttrs (
        map (system: {
          name = system;
          value = (forSystem system).packages;
        }) systems
      );

      apps = builtins.listToAttrs (
        map (system: {
          name = system;
          value = (forSystem system).apps;
        }) systems
      );
    };
}
