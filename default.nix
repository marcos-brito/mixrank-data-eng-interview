let
    pkgs = (import (builtins.fetchTarball {
        url = "https://github.com/NixOS/nixpkgs/archive/3de8f8d73e35724bf9abef41f1bdbedda1e14a31.zip";
        sha256 = "1akf8bq8i1flj33sdwl95lkng4wgwlzh9yxylk2drq8fksg5i291";
    }) { });
    stdenv = pkgs.stdenv;
in pkgs.mkShell rec {
    name = "interview";
    shellHook = ''
        source .bashrc
        export LD_LIBRARY_PATH=${pkgs.openssl.out}/lib:$LD_LIBRARY_PATH
    '';
    buildInputs = (with pkgs; [
        bashInteractive
        cargo
        rustc
        openssl
    ]);
}
