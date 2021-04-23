# sniper

the core component of a sniper: cross-editor snippet manager.

**WARNING**: this is very early, and implementation details will definitely change as I work to implement this in vscode. 

## Description 


Sniper is an editor agnostic snippet manager. The snippet syntax will be a superset of that defined by the [LSP's snippet syntax specification](https://github.com/microsoft/language-server-protocol/blob/master/snippetSyntax.md), currently existing snippets will require a slight reordering(organized by prefix/trigger rather than name), but I'm likely to change back to being organized by name rather than trigger. This, among a few other changes would allow existing vscode snippets to be used.

The project is composed of different components:

### Found here

- **Sniper**
  - the daemonized application
  - handles deserialization and storage of snippets
  - handles tracking of what snippets are grouped together
  - defines logic for state handling
- **Sniper-{LANGUAGE}**
  - these provide functionality to a given language to be implemented in the editor
  - rather than providing sniper as a lib, these provide a mediator for establishing communication between the sniper application and the editor
  - the first target is node via neon-rust, possibly followed by python(via pyO3)
    - may not provide a python wrapper unless there is an editor that requires it for plugins

### Located Elsewhere

- **Sniper-{EDITOR}**
  - written in whatever language is either easiest to write or implement for that editor
  - first target is vscode via typescript, then kakoune
  - actually handles state, such as tracking user input and deciding when to suggest/insert a snippet.
  - planning on leveraging interaction with the language server for the target language in order to have smarter loading or completion. we'll see how it goes
- **Snippets**
  - modular
    - can import snippet sets at runtime, either by command or (hopefully) automatically based off context
    - snippets can be composed of snippets, lazy loaded(rebuilt to perfection when called)
    - (Considering) snippets can be overridden 
  - contextual
    - planned support for multilanguage context. for example, loading mathjax snippets when in the target is a markdown file, or importing html if the current context sometimes calls for embedded html. jupyter notebooks is another good example of when multilanguage snippets is useful. 
    - snippets can be conditionally disabled or enabled based off activity: hopefully, no more annoying suggestions for else unless you have a proceeding if.
  
