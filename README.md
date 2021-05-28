# sniper

the core component of a sniper: cross-editor snippet manager.

**WARNING**: this is very early, and implementation details will definitely change as I work to implement this in vscode. 

- here's what is left to do before it's ready for it's first actual release:
  
  
  - [x] implement first client lib for javascript/typescript with neon
  - [ ] figure out how to cache socket connection to avoid initializing client per function call
  - [ ] switch to file based logging
  - [ ] implement variables and transforms
  - [ ] implement per snippet request parsing
  - [ ] add struct for snippetdata(description and name), turn into jsobject in sniper-node
  - [ ] implement get snippet in vscode(WIP)
  - [ ] figure out how to handle communication with target's LSP
  - [ ] implement missing basic server calls(such as `get_library`)


## Update on progress

When the client and server are ran in release mode, getting completions on each keystroke is FAST, even with the current necessity to initialize the client connection on each function call. 

I'm working on a way to keep a consistent client connection either per editor or per target, and this will greatly reduce the amount of work necessary both client and server side. I'm also about to switch from print statements to actual file based logs, so that it will be possible to daemonize the server(on first connection request), which is currently impossible due to daemons not having anything attached to their STDOUT. this will make it possible to daemonize the server on first added target, and the server will shut itself down when the last target is dropped. 

plus, I'm about to implement true server side parsing which, in order to keep this truly editor agnostic, means adding variables into the mix.
## Description 

Sniper is an editor agnostic snippet manager. The snippet syntax is currently superset of that defined by the [LSP's snippet syntax specification](https://github.com/microsoft/language-server-protocol/blob/master/snippetSyntax.md). While right now the snippet directory is static (located at `~/.config/sniper`), this means existing vscode snippets are compatible.

The project is composed of different components:

### Found here
- **sniper-common**
  - defines the RPC that is used by the server and all client implementations
  - will also likely store some common client functionality
- **Sniper**
  - the backend-server
  - communicates with client over unix domain socket
  - started automatically on first client request
  - handles deserialization and storage of snippets
  - handles tracking of what snippets are grouped together
  - defines logic for state handling
### Located Elsewhere
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
  
