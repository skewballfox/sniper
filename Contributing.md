
TODO: coming back to this later

## Setting up Jaegar Tracing
TODO

## Project components:

#### Found here

note: this is definitely going to change soon, as I'm moving to another way of handling RPC, namely gRPC.

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
  
