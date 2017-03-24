# SoTo - Source Tooling
Various tools for working with Source Engine assets.

*SoTo and Carbide Games are not associated with Valve, this is a third party
toolset.*
*This project is under heavy development and it is very likely that it won't work
for you.*

**SoTo is made with FBX 2016/2017 as reference, other versions may not work.**

## Why SoTo
So you're working on your next big hat project, it's at least 20% more hat than
your previous hats, why should you use SoTo? Unlike common 3D editor plugins
SoTo is designed with the following ideals:

- Editor Agnostic - Using the widely supported FBX format you can use nearly any
    editor you want without the need for a plugin specifically for that editor.
- Repeatable - No need to remember export settings and be careful around the
    export window. As long as you have the FBX files and the SoTo files, you can
    re-build your assets whenever you want.
- Made to be Versioned - SoTo is made with version and source control in mind.
    It can very easily be used with your choice of version control.

## Why **not** SoTo
SoTo is still in heavy development, **it's very likely that it will not work for
you**. Many FBX layouts are not supported, which may cause it to not work with
some editors' exported files. If you want to contribute to change this, feel
free to fork the code and submit a pull request.

## Projects

### soto
A Source Engine asset project manager and build tool.

### soto-fbx
A plugin for soto-cli that converts FBX files to Source Engine MDLs.

### sotolib
Various support libraries used by SoTo. Contains a FBX parser and a SMD writer.

## License
Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
