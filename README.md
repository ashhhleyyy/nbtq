# nbtq

Convert NBT files into JSON for easy processing with tools like `jq`

## Installing

`nbtq` can be installed using `cargo`:

```
cargo install nbtq
```

## Usage examples

Extract a list of regions from a [plasmid](https://github.com/NucleoidMC/plasmid) map template:

```
nbtq map.nbt | jq .regions[].marker
```

Pretty print NBT to a file:

```
nbtq --pretty example.nbt > example.json
```
