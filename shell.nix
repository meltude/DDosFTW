{
  pkgs ? import <nixpkgs> { },
}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    go
    gopls
    gnumake
  ];

  shellHook = ''
    go version
    echo "Autobuilding project..."
    make build
  '';
}
