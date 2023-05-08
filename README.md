# nugget

It's like `nuget`, but more g.

Essentially a wrapper around the `dotnet` CLI to enable it to do the things it **should** do already.

This is a WIP. 

Features so far:

- Interactive updating of nuget packages across multiple projects (under one solution file).
- Automatic updating of all nuget packages.
- Filter nuget packages by a prefix.

To install, run `cargo install nugget`.

To use, ensure you are in the folder containing the dotnet project you would like to update, and run `nugget`. This will trigger an interactive process where you can select which nuget packages to update. For more usage details, run `nugget --help`.

## examples

1. Interactively update all outdated dependencies
```
nugget
```

2. Automatically update all outdated dependencies
```
nugget --auto
```

3. Interactively update outdated dependencies with one of the specified prefixes
```
nugget --prefixes Mongo Redis
```

4. Automatically update outdated dependencies with one of the specified prefixes
```
nugget --auto --prefixes Mongo Redis
```
