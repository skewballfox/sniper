
# Contributing

## Project components

If you want to contribute something to the project, but aren't sure where to go, this is a layout of the pieces of the project, and what each piece is responsible for.

### Found here

#### Sniper

  the backend-server which:

- communicates with client over unix domain socket
- is started automatically on first client request(WIP)
- handles deserialization and storage of snippets
- handles tracking of what snippets are grouped together, what files(ie targets) they are being used by
- provides completions on client request(per keystroke)

#### sniper-client

a test client without an associated editor used to verify things are working with the server

### Located Elsewhere

#### Sniper-{EDITOR}
  
  These are the editor extensions which interact with sniper server. It may or may not make sense to add libs for a given language, depending on how many editors use a given language for extensions. In most cases as most of the logic for handling a given task is implemented server-side, client-side handling boils down to taking the results of the server and using that specific editors api to serve up the completed result, and the bindings for the rpc are generated from the proto files, there probably isn't much to gain from implementing a library for this in a given language.

  the clients are:

- written in whatever language is either easiest to write or implement for that editor
- only focused on being the glue layer between the server and the editor
  - if it takes more logic than output data structure -> expected data structure, it probably belongs elsewhere.
- planning on leveraging interaction with the language server for the target language in order to have smarter loading or completion. we'll see how it goes
  
#### Snippets and Functors

  Snippets are the individual pieces of static text which will be manipulated to insert a requested completion.
  Functors are the things which serve a programmatic result. These are either one-off changes(in case of variables) or dynamic changes likely related to the input of placeholders/tabstops, This way variable support isn't hard coded, and can expand over time, or shrink depending on need and availability. This also allows for programmatic snippets in an editor agnostic way.

- modular
  - can import snippet sets at runtime, either by command or (hopefully) automatically based off context
  - (Considering) snippets can be overridden
- contextual
  - planned support for multilanguage context. for example, loading mathjax snippets when in the target is a markdown file, or importing html if the current context sometimes calls for embedded html. jupyter notebooks is another good example of when multilanguage snippets is useful.
  - snippets can be conditionally disabled or enabled based off activity: hopefully, no more annoying suggestions for `else` unless you have a proceeding `if`.
  - [here's the python snippet file I've been using to develop the project](https://github.com/skewballfox/sniper/blob/master/config/python/base.json)

## Setting up Jaegar Tracing

TODO
