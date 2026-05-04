{
  pkgs,
  rustToolchain,
}:
pkgs.mkShell {
  packages = with pkgs; [
    rustToolchain
    alejandra
  ];
}
