# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "stable-23.11"; # or "unstable"

  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.docker
    pkgs.docker-compose
    pkgs.cmake          # For CMake
    pkgs.gcc            # For C/C++ compilation
    pkgs.gtest          # GoogleTest
    pkgs.hiredis        # Hiredis
    pkgs.meson          # Meson build system
    pkgs.ninja          # Ninja build system
    pkgs.git            # Version control
    pkgs.gnumake
    pkgs.python3
    pkgs.hiredis
    pkgs.gtest
    pkgs.pkg-config      # For package configuration
  ];

  # Sets environment variables in the workspace
  env = {};
  # Enable Docker
  services.docker = {
    enable = true;
  };

  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      # "vscodevim.vim"
       "cliffordp.vscode-cpp"       # C/C++ extension
    ];

    # Enable previews
    previews = {
      enable = true;
      previews = {
        # web = {
        #   # Example: run "npm run dev" with PORT set to IDX's defined port for previews,
        #   # and show it in IDX's web preview panel
        #   command = ["npm" "run" "dev"];
        #   manager = "web";
        #   env = {
        #     # Environment variables to set for your server
        #     PORT = "$PORT";
        #   };
        # };
      };
    };

    # Workspace lifecycle hooks
    workspace = {
      # Runs when a workspace is first created
      onCreate = {
        # Example: install JS dependencies from NPM
        # npm-install = "npm install";
      };
      # Runs when the workspace is (re)started
      onStart = {
        # Example: start a background task to watch and re-build backend code
        # watch-backend = "npm run watch-backend";
      };
    };
  };
}
