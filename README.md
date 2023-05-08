# nugget

It's like nuget, but more g.

Essentially a wrapper around the `dotnet` CLI to enable it to do the things it **should** do already.

This is a WIP. Features:

- [x] Interactive updating of nuget packages across multiple projects
- [x] Automatic updating of all nugets
- [x] Filter nugets by a prefix
- [ ] Make terminal output a little more palateable

I'm too lazy to publish this on crates.io just yet. To install, `git clone https://github.com/emilioziniades/nugget && cd nugget && cargo install --path .`.

To use, ensure you are in the folder containing the dotnet project whose nugets you would like to update, and run `nugget`. This will trigger an interactive process where you can select which nugets to update. For more usage details, run `nugget --help`.

## examples

1. Interactively update all outdated dependencies:
```
nugget
```

2. Automatically update all outdated dependencies:
```
nugget --auto
```

3. Interactively update outdated dependencies with one of the specified prefixes:
```
nugget --prefixes Mongo Redis
```

4. Automatically update outdated dependencies with one of the specified prefixes:
```
nugget --auto --prefixes Mongo Redis
```
