# sniper

the core component of a sniper: cross-editor snippet manager.

**WARNING**: this is very early, and implementation details will definitely change as I work to implement this in vscode. 

- here's what is left to do before it's ready for it's first actual release:
  
  
  - [x] implement first client lib for javascript/typescript with neon
  - [ ] implement in vscode with sniper-node (WIP)
  - [ ] implement missing basic server calls(such as `get_library`)
  - [ ] refactor/improve once I have a minimal working state

## Update on progress

I'm doing a bit of research on similar editor extensions to figure out what function calls are necessary, and working on a way to make much of the work editor independent.

with any luck implementing this in a given editor will be a matter of implementing function calls
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
  - started automatically on first client request thanks to systemd
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
    - snippets can be conditionally disabled or enabled based off activity: hopefully, no more annoying suggestions for else unless you have a proceeding if.
  
