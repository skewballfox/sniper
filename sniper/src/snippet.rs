use crate::target::Target;
use std::collections::HashSet;
use std::rc;
use serde::Deserialize;
//these are the currently supported actions for snippets
#[derive(Deserialize, Clone, Debug)]
#[serde(tag = "action", content = "args")]
pub enum Actions {
    Load(Vec<String>),
    Enable(Vec<String>),
    Disable(Vec<String>),
    //Commands, //potentially script running of commands such as making or renaming a file
    //Adjust, //shouldn't be in file, there to tell sniper to reparse at snippet launch time
}
#[derive(Deserialize, Clone, Debug)]
#[serde(rename = "type")]
pub enum SnippetTypes {
    Shorthand,
    Statement,
    Expression,
    Template,
}

#[derive(Debug)]
pub struct SnippetSets {
    /// tracks the set each group of snippets belong to, as well as
    /// which targets require them
    snippets: Vec<String>,
    //TODO: may want to add methods on all structs using weak to occasionally clean references
    required_by: HashSet<rc::Weak<Target>>,
}

//TODO: consider implementing snippet as a type rather than a struct
// would be combined with a match at runtime to execute appropriate behavior
#[derive(Deserialize, Clone, Debug)]
pub struct Snippet {
    name: String,
    #[serde(rename = "type", default = "default_snippet_type")]
    snippet_type: SnippetTypes,
    body: Vec<String>,
    description: String,
    #[serde(default = "unconditional")]
    is_conditional: bool,
    #[serde(default = "no_action")]
    actions: Vec<Actions>,
}

fn default_snippet_type() -> SnippetTypes {
    SnippetTypes::Shorthand
}

fn unconditional() -> bool {
    false
}

fn no_action() -> Vec<Actions> {
    Vec::new()
}

#[derive(Deserialize, Clone, Debug)]
pub struct Loader {
    #[serde(flatten, with = "tuple_vec_map")]
    snippets: Vec<(String, Snippet)>,
}