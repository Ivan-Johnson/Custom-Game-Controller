{
	description = "Packages for my custom game controller";

	inputs = {
		nixpkgs.url = "nixpkgs/nixos-25.11-small";
		fenix = {
			url = "github:nix-community/fenix";
			inputs.nixpkgs.follows = "nixpkgs";
		};
	};

	outputs =
		{
			self,
			nixpkgs,
			fenix,
		}:
		let
			pkgs = import nixpkgs {
				system = "x86_64-linux";
				config.allowUnfreePredicate = pkg: builtins.elem (nixpkgs.lib.getName pkg) [ "vscode" ];
			};
			rustPlatform = pkgs.rustPlatform;
		in
		let
			shell = pkgs.mkShell {
				buildInputs = [
					pkgs.arduino-cli
					pkgs.arduino-ide
					pkgs.avrdude
					pkgs.blender
					pkgs.pkgsCross.avr.buildPackages.gcc
					(pkgs.python3.withPackages (python-pkgs: with python-pkgs; [ pyserial ]))
					pkgs.minicom
					pkgs.ravedude
					pkgs.vscode
					(fenix.packages.x86_64-linux.fromToolchainFile {
						file = ./rust-toolchain.toml;
						# sha256 = "sha256-z8J/GH7znPPg9kKvPirKcBeXqHikj1M7KB+anwsDx0M=";
						sha256 = "sha256-cQl292Ia+Crg9ps29Pv5ciufXd0b/HF7770/bOEDv+k="; # aria 2025-12-29T23:52:57-05:00
					})
				];
				RAVEDUDE_PORT = "/dev/ttyACM0";
			};
		in
		{
			devShells.x86_64-linux.default = shell;

			# This is kinda a hack. I'm not even sure if it works.
			#
			# Setting `shell` as the default target means that `nix
			# build` will build the shell environment and create
			# `result` (a sym link to the activation script).
			#
			# As I understand it, as long as that symlink exists
			# the garbage collector will never delete the shell
			# environment from the nix store. In particular, this
			# means that `nix develop` will always be fast.
			packages.x86_64-linux.default = shell;
		};
}
