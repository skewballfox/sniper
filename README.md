# sniper-core

the core component of a sniper: cross-editor snippet manager.

**WARNING**: this is very early, and implementation details will definitely change as I work to implement this in vscode. 

## Description 

### of Sniper as a whole

Sniper is a (hopefully) editor agnostic snippet manager. The snippet syntax will be a superset of that defined by the [LSP's snippet syntax specification](https://github.com/microsoft/language-server-protocol/blob/master/snippetSyntax.md), existing snippets will require a slight reordering(organized by prefix/trigger rather than name), but this project is aiming to make migration as easy as possible for new users.

The project is composed of different components:

## Found here
- **Sniper**
    - the (likely) daemonized application
    - handles deserialization and storage of snippets
    - handles tracking of what snippets are grouped together
    - defines logic for state handling
- **Sniper-{LANGUAGE}**
    - these provide functionality to a given language to be implemented in the editor
    - rather than providing sniper as a lib, these provide a mediator for establishing communication between the sniper application and the editor
    - the first target is node via neon-rust, followed by python(via pyO3)
- **Sniper-{EDITOR}**
    - written in whatever language is either easiest to write or implement for that editor
    - first target is vscode via typescript, then maybe kakoune via python
    - actually handles state, such as tracking user input and deciding when to suggest/insert a snippet.
    - may leverage interaction with the language server for the target language in order to have smarter loading or completion

### of sniper core

sniper-core is the heart of sniper. Snippets are stored in a trie mainly for fast prefix checking. most of information that will be needed in order to have better autocompletion suggestions will be stored here, but doesn't try to actually maintain state so much as make it easier to keep track of it. The purpose of sniper core is to store the relevant snippets, retrieve the and manipulate the contents when requested and leave the determination of appropriate context to someone else, namely the program interacting with the editor(and hopefully the language server)