# nix build, then link the plugins where your DAW looks:
#   ln -s "$PWD/result/lib/vst3/Tearout Farm.vst3" ~/.vst3/
#   ln -s "$PWD/result/lib/clap/Tearout Farm.clap" ~/.clap/
{
  description = "Tearout Farm";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }:
    let
      arch = { x86_64-linux = "x86_64-linux"; aarch64-linux = "aarch64-linux"; };
      build = system: let pkgs = nixpkgs.legacyPackages.${system}; in pkgs.stdenv.mkDerivation {
        pname = "tearout-farm";
        version = "0.1.0";
        src = ./.;
        nativeBuildInputs = [ pkgs.autoPatchelfHook ];
        buildInputs = [ (pkgs.libx11 or pkgs.xorg.libX11) (pkgs.libxcb or pkgs.xorg.libxcb) pkgs.libGL ];
        dontBuild = true;
        dontStrip = true;
        installPhase = ''
          so="Tearout Farm.vst3/Contents/${arch.${system}}/Tearout Farm.so"
          mkdir -p "$out/lib/vst3/Tearout Farm.vst3/Contents/${arch.${system}}" $out/lib/clap
          cp "$so" "$out/lib/vst3/$so"
          cp "$so" "$out/lib/clap/Tearout Farm.clap"
        '';
      };
    in {
      packages = builtins.mapAttrs (system: _: { default = build system; }) arch;
    };
}
