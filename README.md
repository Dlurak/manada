# Manada

Extensible graph based unit conversion cli.

## Concept

If it is known how to convert from centimeters to decimeters to meters it automatically also known how to convert from centimeters to meters. 
Manada uses this graph based approach to convert from many units. The actual conversion formulas can be written in simple text files. 

## Installation

You can clone this repository and build it using cargo.

### Nix

This repo provides a flake, this flakes provides a nixosModule which you can important and enable using `programs.manada.enable = true`. It applies the config files in `/etc` and allows you to write them in nix. There is also a home-manager module, it applies the config files in `~/.config/manada`.

