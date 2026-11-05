# DDosFTW

<img src="docs/image.webp" alt="screenshot" width="400">

---

_this is fake hacking server/accouts program just for trolling, nothing more_

_it does absolutetly nothing, just messing with output_

_u can troll people with it_

---

## Modes:

- `[1] domain hack` — fake domain takeover sequence
- `[2] account hack` — fake account takeover with session, device and profile details
- `[3] de-anonymise` — fake de-anonymisation report (profile correlation, location and confidence guess)

everything is fabricated: no network access, no real lookups, no data collection.

## Building:

for now make sure that u have all dependecies that required for this program

```packages
rust & make
```

lets begin with building using GNU Make:

```bash
cd DDosFTW
make
```

easy right?

so here the much harder way:

```bash
cd DDosFTW
cargo build --release
```

and for my favorite NixOS users:

```bash
cd DDosFTW
nix develop
```

or build it with the flake:

```bash
cd DDosFTW
nix build
./result/bin/DDosFTW
```

the flake uses flake-parts and works on x86_64/aarch64 Linux and Apple Silicon macOS.
