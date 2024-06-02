_default:
  just --list

check:
  nix flake check

build:
  just check
  nix build

run *args:
  just build
  ./result/bin/kdlfmt {{args}}
