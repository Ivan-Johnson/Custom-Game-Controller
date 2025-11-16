{
	description = "driver";

	inputs.nixpkgs.url = "nixpkgs/nixos-25.05-small";

	outputs =
		{ self, nixpkgs }:
		let
			pkgs = import nixpkgs { system = "x86_64-linux"; };
			rustPlatform = pkgs.rustPlatform;
			driver = rustPlatform.buildRustPackage {
				pname = "driver";

				version = "0.1.0";

				src = ./.;

				buildInputs = [ pkgs.makeWrapper ];

				checkFlags = [
					#"--skip=foo::bar::..."
				];

				cargoLock.lockFile = ./Cargo.lock;
			};
		in
		{
			devShells.${pkgs.system}.default = pkgs.mkShell {
				buildInputs = [
					pkgs.arduino-ide
					pkgs.blender
					pkgs.cargo
					pkgs.cargo-flamegraph
					pkgs.clippy
					pkgs.lldb
					pkgs.rustc
					pkgs.rustfmt
				];
			};

			# temporarily changed to GNU's Hello World package
			packages.${pkgs.system}.default = pkgs.hello;

			nixos_options =
				{
					config,
					lib,
					pkgs,
					...
				}:

				let
					cfg = config.programs.driver;
				in
				{
					options = {
						programs.driver = {
							enable = lib.mkEnableOption "driver";
						};
					};

					config = lib.mkMerge [ (lib.mkIf cfg.enable { home.packages = [ driver ]; }) ];
				};
		};
}
