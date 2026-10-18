# nix build, then link the plugins where your DAW looks:
#   ln -s "$PWD/result/lib/clap/Tearout Farm.clap" ~/.clap/
#   ln -s "$PWD/result/lib/vst3/Tearout Farm.vst3" ~/.vst3/
{
  description = "Tearout Farm";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux; in {
      packages.x86_64-linux.default = pkgs.stdenv.mkDerivation {
        pname = "tearout-farm";
        version = "0.1.0";
        src = ./.;
        nativeBuildInputs = [ pkgs.autoPatchelfHook ];
        buildInputs = [ (pkgs.libx11 or pkgs.xorg.libX11) (pkgs.libxcb or pkgs.xorg.libxcb) pkgs.libGL ];
        dontBuild = true;
        dontStrip = true;
        installPhase = ''
          mkdir -p $out/lib/clap $out/lib/vst3
          cp "Tearout Farm.clap" $out/lib/clap/
          cp -r "Tearout Farm.vst3" $out/lib/vst3/
        '';
      };
    };
}
