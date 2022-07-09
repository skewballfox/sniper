# sniper

the hopefully cross-platform, potentially editor agnostic, probably over-ambitions and definitely over-engineered snippet management server

**WARNING**: this is very early, and implementation details will definitely change as I work to implement this in vscode.

## Description

Sniper is an editor agnostic stateful snippet manager. The snippet syntax is currently superset of that defined by the [LSP's snippet syntax specification](https://github.com/microsoft/language-server-protocol/blob/master/snippetSyntax.md). While right now the snippet directory is static (located at `~/.config/sniper`), this means existing vscode snippets are compatible.

### (When its done) Why Use Sniper?

#### TL; DR

- you can write (hopefully programmatic) snippets that have the largest user base
- it's one less thing you have to set up when trying out a new editor, or writing your own
- snippets are
  - modular: you can load and unload sets as you need them
  - composable: you can make snippets out of other snippets,
  - programmatic, still working out the details, but I plan on using grpc streaming along with scripting support(wasm?python? your guess is as good as mine.)
  - and (eventually) package-based and hands-off: just install the server and don't worry about the rest, unless you want to

my goal since the beginning of this project has been to make a system that goes beyond providing basic autocompletion and support context aware dynamic code generation.  

#### Ok, now I'm interested

IMO, part of the reason snippets aren't more widely used is because of the individual effort involved. You could install the base snippets you have for your editor, but they aren't really flexible enough to justify putting a lot of time into creating custom ones. With snippet managers like [ultisnips](https://github.com/SirVer/ultisnips) or [hsnips](https://github.com/draivin/hsnips)(which I feel are powerful enough to justify use) if you make snippets for those tools, their utility is limited to a small subset of developers using the same editor and plugin. In both cases suggestions and the list of options isn't changed by file/project specific context, such as whether a certain library is imported. if you snippets for numpy, other than swapping out the snippets being loaded, they will always remain suggested. This project is set up to load a base set of snippets, with the ability to load snippet libraries as needed (hopefully automatically depending on what I can get from the in play language server).

Currently I'm working towards supporting a superset of the LSP snippet syntax, which by itself isn't much, but will make it rather easy to implement snippets in new editors (rather than building the functionality inside the editor or a plugin for the editor), and also makes it easy to use your current vscode snippets across editors(once they are implemented).

The plan is to support scriptable/programmatic snippets, i.e. the type of thing that makes vim's ultisnips such a powerful tool. This, when combined with library-based snippet sets(another planned supported feature), will allow for an autocompletion system that is both powerful and flexible

Also lastly, and arguably most important, I want to support some system of snippet sharing/distribution whether through simple github gist or something like cargo crates. my goal is to eventually make it to where you could run something like `sniper --install-recommended` and download snippets for the languages **and the libraries** which you have on your system.

As an example it's easy to build an rpc server with libraries like tonic and tarpc, but I want to make it to where you can do so with minimum keystrokes, in a way that supports the contextual nuance required to build one for your specific project.

## Installation

first build the server and place it somewhere on your path

```bash
git clone https://github.com/skewballfox/sniper && \
cd sniper && \
cargo build --release -p sniper-server && \
cp ./target/release/sniper $HOME/.local/bin/ 
```

ps: if you don't want to actually put it along your path and just want to test the server, then copy the first 3 lines (excluding the last `&`) and then run `cargo run -p sniper-server`

otherwise you can then launch the server

```
sniper
```

then it's ready to use, for example with the test client:

```
cargo run -p sniper-client
```
