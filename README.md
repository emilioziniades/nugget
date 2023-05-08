# nugget

It's like nuget, but with more g.

Essentially a wrapper around the `dotnet` CLI to enable it to do the things it **should** do already.

This is a WIP. Features:

- [x] Interactive updating of nuget packages across multiple projects
- [ ] Automatic updating of all nugets
- [ ] Filter nugets by a prefix
- [ ] Make terminal output a little more palateable

So far, just change into a dotnet project with a solution file, and run `nugget`. This will trigger an interactive process where you can select which nugets to update.
