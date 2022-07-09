# Snippet specification and syntax

- [Snippet syntax specification defined by the LSP](https://github.com/microsoft/language-server-protocol/blob/master/snippetSyntax.md)
- since I'm trying to work toward feature parity with this specification, I will only discuss the parts of this solution that aren't covered in this specification.

## Added Syntax

- Snippet
  - prefixed by `$!`, must be a valid snippet currently in the same file
    - ex: `$!if`
  
## Components of Snippets

### name

  the name of the snippet

### description

  the description of the snippet

### prefix

the trigger to be used for completion suggestions

### body

- the content of the snippet
- must be an array even if only one element
  - may change as I get more familiar with serde
- snippets can be composed of other snippets

### is conditional (CURRENTLY UNUSED)

- whether or not the snippet should always be suggested completed or should only be suggested when enabled
- if it isn't defined, the snippet is assumed to be unconditional and will always be enabled

### actions (CURRENTLY UNUSED)

- these are things for sniper to do whenever the snippet is triggered (see [snippet actions](#snippet-actions)). currently these are planned to be acted on prior to insertion
- only need to be defined when you want to attach actions to a snippet

## Snippet Actions (UNIMPLEMENTED)

### Current

#### Enable

- in order to have smarter suggestions, and prevent frustrations when classes of snippet types are set to autotrigger, some components need to be suggested/inserted only when they are enabled, and ignored otherwise such as statements `else` and `finally`. Which would be enabled by `if`,`for`,`while`,`try` and `try` respectively.

#### Disable

- to follow the same example, on the flip side, these need to be disabled when they are triggered or once the scope has been left.

#### Load

- In order to make snippets easier to learn and share, and make suggestions more useful and less annoying/distracting, the list of available snippets needs to be dynamic.
- the intention is for every supported language to have
  - a base set which are always loaded,
  - library specific sets which will loaded when importing that library
  - followed by clustered library sets(snippets that involve two or more libraries), still working out how that would work, other than the user just overtly telling sniper to load.
  - useful clusters of snippets, that the user wants to either only explicitly load or tie only to some project/workspace
- I'm still not sure how to handle snippet distribution, exploration, etc. obviously I want some way for users to compare snippets, try out new snippets. I know that there will have to be some kind of core set, the basic functionality for that language (gradually expanding to more of the standard library), and library specific sets (such as computer vision templates if you have opencv installed), but how this would work is something I don't have enough experience to work out.
- Off topic but this is actually the functionality that I find lacking in almost every snippet solution and this is part of the reason I actually set out to write yet another snippet manager, and also the reason why I think it's necessary for the solution to be editor agnostic. There have been some truly amazing snippet managers out there that made progress towards really good autocompletion systems, that are then "rediscovered" by the next code monkey that tackles the same problem in their editor of choice. I think library based snippets would be a natural consequence of a lot of programmers from different domain spaces all using the same tool to make whatever they were writing faster, with minimal reinvention necessary.

## Things I'm considering

### Snippet Types

- may either be impossible or require a lot of work per editor, so it's in the maybe one day category
- essentially getting language context to allow for smarter suggestions
- would rely on something like an lsp or parse tree to get such context

### Templates

- one off snippets that are only valid in empty files, basically there for super fast boilerplating

### Command Action

- this is the use case I'm considering:
  - you have created an abstract base class, and are implementing it in a child class, you can avoid writing boilerplate by reading the appropriately named file, parsing it's content, and outputting the modified method stubs into the current file/class (Zero Tedium object orientation!), you still need to obviously add content to the methods, but rewriting the same function declaration with extra steps won't be necessary.
  - you are working on a hexogonal architecture based program and write a port in the domain logic, a file can be generated in an appropriate directory that when fleshed out will be the corresponding adapter.
  - you have created an empty file and you have no idea what you want to build. you decide to build an OpenCV application, and launch a generic enough template. The file is named as genericCV.py and saved either in a directory you have specified as a catch all or whatever directory you launched the editor in, until you figure out what else it should be named.
  - generating stubs for test when writing functions/methods or vice versa
- obviously this has the potential to be dangerous but I think it could have a lot of potential for improving the development speed via snippets.

## Keywords (UNIMPLEMENTED)

the goal is to support all the keywords defined in the specification already listed so I won't cover those. instead I'll just list the ones unique to sniper.

- `LIB`
  - in order to have multiple libraries, and to avoid overwriting standard snippets, it makes sense to prefix library snippets with some kind of prefix, set at library load time
    - for some libraries this is an obvious choice determined by standard practices, such as naming `numpy` as `np`, others it's not an obvious choice
      - you don't want the library name to be limited to being the same as the prefix because the prefix is meant to be quick to type, which may not be clear to read
      - you don't want to set this to be static because what if a user directly imports a sub-module? you need some way to refer to the library, separate from the storage prefix(which could be set to a default by the user)
      - my initial idea is to store snippets for libraries composed of modules as a stack of directories
      - another slightly more elegant idea is to store large library in a single directory with snippets organized by submodules, or variations of the same set of snippets where the one loaded is determined by scope
  - obviosly still working out the details, I'm probably not going to have a good idea how to do this, until I solve the more immediate problems and work on expand the snippets I'm testing things with
  - it might make sense to adopt a strategy similar to python black, making the naming fairly opinionated based on what is idiomatic or standard practice, though if given the option I would want usage to be dynamically related to user behavior rather than the other way around.
  - depending on what kind of information I can get from the language server on the current session this may be a really easy or really interesting challenge.
  