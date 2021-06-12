# sniper

the core component of a sniper: cross-editor snippet manager.

**WARNING**: this is very early, and implementation details will definitely change as I work to implement this in vscode. 

- here's the bare minimum necessary for a first release:  
  - [x] implement first client lib for javascript/typescript with neon
  - [x] add struct for snippetdata(description and name), turn into jsobject in sniper-node
  - [x] implement `get_completions` and `get_snippet` in [vscode](https://github.com/skewballfox/sniper-code)
  - [x] implement tracing
  - [ ] setup sniper to daemonize on initial request via a commandline flag (WIP)
  - [ ] implement some sort of setup or installation script



there's a lot more to handle after this, but at that point it will be pretty safe to:

1. comfortably use this tool
2. implement sniper in other editors with the expectation of (only) some growing pains

## Update on progress

I have working completions and snippets for vscode. granted it's technically not cross editor at the moment, because it relies on the definition of a vscode [snippetString](https://vshaxe.github.io/vscode-extern/vscode/SnippetString.html).

I'm working towards daemonizing the server(planned on first connection request), but running into issues with requests being dropped, but once I solve that it should be trivial to start the server as needed.

This is getting closer to being ready for use in the wild, Honestly kind of excited for that.

## Description 

Sniper is an editor agnostic snippet manager. The snippet syntax is currently superset of that defined by the [LSP's snippet syntax specification](https://github.com/microsoft/language-server-protocol/blob/master/snippetSyntax.md). While right now the snippet directory is static (located at `~/.config/sniper`), this means existing vscode snippets are compatible.

### (When its done) Why Use Sniper? 

IMO, part of the reason snippets aren't more widely used is because of the individual effort involved. You could install the base snippets you have for your editor, but they aren't really flexible enough to justify putting a lot of time into creating custom ones. With snippet managers like [ultisnips](https://github.com/SirVer/ultisnips) or [hsnips](https://github.com/draivin/hsnips)(which I feel are powerful enough to justify use) if you make snippets for those tools, their utility is limited to a small subset of developers using the same editor and plugin. In both cases suggestions and the list of options isn't changed by file/project specific context, such as whether a certain library is imported. if you snippets for numpy, other than swapping out the snippets being loaded, they will always remain suggested. This project is set up to load a base set of snippets, with the ability to load snippet libraries as needed (hopefully automatically depending on what I can get from the in play language server).

Currently I'm working towards supporting a superset of the LSP snippet syntax, which by itself isn't much, but will make it rather easy to implement snippets in new editors (rather than building the functionality inside the editor or a plugin for the editor), and also makes it easy to use your current vscode snippets across editors(once they are implemented).

The plan is to support scriptable/programmatic snippets, i.e. the type of thing that makes vim's ultisnips such a powerful tool. This, when combined with library-based snippet sets(another planned supported feature), will allow for an autocompletion system that is both powerful and flexible

Also lastly, and arguably most important, I want to support some system of snippet sharing/distribution whether through simple github gist or something like cargo crates. my goal is to eventually make it to where you could run something like `sniper --install-recommended` and download snippets for the languages **and the libraries** which you have on your system.

### Project components:

#### Found here

- **sniper-common**
  - defines the RPC that is used by the server and all client implementations
  - will also likely store some common client functionality
- **Sniper**
  - the backend-server
  - communicates with client over unix domain socket
  - started automatically on first client request(WIP)
  - handles deserialization and storage of snippets
  - handles tracking of what snippets are grouped together, what files(ie targets) they are being used by
  - provides completions on client request(per keystroke)

#### Located Elsewhere

- **Sniper-{LANGUAGE}**
  - these provide functionality to a given language to be implemented in editor's where rust can't be used directly
  - rather than providing sniper as a lib, these provide a mediator for establishing communication between the sniper(the server) and the editor
  - [here's the client lib for javascript/typescript](https://github.com/skewballfox/sniper-node)
- **Sniper-{EDITOR}**
  - written in whatever language is either easiest to write or implement for that editor
  - planning on leveraging interaction with the language server for the target language in order to have smarter loading or completion. we'll see how it goes
  - [here's the in progress vscode extension](https://github.com/skewballfox/sniper-code)
- **Snippets**
  - modular
    - can import snippet sets at runtime, either by command or (hopefully) automatically based off context
    - snippets can be composed of snippets, lazy loaded(rebuilt to perfection when called)
    - (Considering) snippets can be overridden 
  - contextual
    - planned support for multilanguage context. for example, loading mathjax snippets when in the target is a markdown file, or importing html if the current context sometimes calls for embedded html. jupyter notebooks is another good example of when multilanguage snippets is useful. 
    - snippets can be conditionally disabled or enabled based off activity: hopefully, no more annoying suggestions for `else` unless you have a proceeding `if`.
    - [here's the python snippet file I've been using to develop the project](https://github.com/skewballfox/sniper/blob/master/config/python/base.json)
  
