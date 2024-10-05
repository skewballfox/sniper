/*
    Structs for storing, representing, and deserializing the snippets currently
    available and ready to use by the server and its clients, most of the fields
    are allowed to be missing during deserialization so that the base syntax is
    compatible with existing vscode snippets
*/
use serde::Deserialize;
use sniper_common::sniper_proto::SnippetInfo;
//these are the currently (planned) supported actions for snippets
#[derive(Deserialize, Clone, Debug)]
#[allow(dead_code)]
#[serde(tag = "action", content = "args")]
pub enum Actions {
    Load(Vec<String>),
    Enable(Vec<String>),
    Disable(Vec<String>),
    //Commands, //potentially script running of commands such as making or renaming a file
}
#[derive(Deserialize, Clone, Debug)]
#[serde(rename = "type")]
pub enum SnippetTypes {
    Shorthand,
    Statement,
    Expression,
    Template,
}

///The snippet that is deserialized from the file, contains data that will be
#[derive(Deserialize, Clone, Debug)]
pub struct RawSnippet {
    #[serde(with = "serde_bytes")]
    pub(crate) prefix: Vec<u8>,
    #[serde(rename = "type", default = "default_snippet_type")]
    pub(crate) snippet_type: SnippetTypes,
    pub(crate) body: Vec<String>,
    pub(crate) description: String,
    #[serde(default = "unconditional")]
    pub(crate) is_conditional: bool,
    #[serde(default = "no_action")]
    pub(crate) actions: Vec<Actions>,
    //TODO: remove once fully migrated to incremental parsing
    #[serde(default = "assembly_required")]
    pub(crate) requires_assembly: bool,
    //TODO: remove if this turns out to not be necessary with incremental parsing
    #[serde(default = "currently_empty")]
    pub(crate) tabstops: Vec<(usize, usize, usize)>,
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
fn assembly_required() -> bool {
    true
}
fn currently_empty() -> Vec<(usize, usize, usize)> {
    Vec::new()
}

#[derive(Deserialize, Clone, Debug)]
pub struct Loader {
    #[serde(flatten, with = "tuple_vec_map")]
    pub(crate) snippets: Vec<(String, RawSnippet)>,
}
/// WIP all the content of RawSnippet that isn't needed when requesting
/// a snippet. Only differs from SnippetInfo by the addition of prefix,
/// which is necessary when constructing new Targets that employ existing sets
#[derive(Debug, Clone)]
pub(crate) struct SnippetMetadata {
    pub(crate) prefix: Vec<u8>,
    pub(crate) name: String,
    pub(crate) description: String,
}

impl SnippetMetadata {
    pub(crate) fn to_snippet_info(self) -> (Vec<u8>, SnippetInfo) {
        (
            self.prefix,
            SnippetInfo {
                name: self.name,
                description: self.description,
            },
        )
    }
    pub(crate) fn get_name(&self) -> String {
        self.name.clone()
    }
}

/// tracks the set each group of snippets belong to, as well as
/// how many targets require them, used to determine whether it's necessary
/// to add or drop a group of snippets
#[derive(Debug)]
pub struct SnippetSet {
    pub(crate) contents: Vec<SnippetMetadata>,
    target_counter: i32,
}

impl SnippetSet {
    pub(crate) fn new(contents: Vec<SnippetMetadata>) -> Self {
        Self {
            contents,
            target_counter: 1,
        }
    }

    pub fn increment_target_count(&mut self) {
        self.target_counter += 1;
    }

    ///this is ran when a dropping a target, the returned boolean is used to
    /// determine if the snippets associated with this snippet set need to be
    /// dropped
    pub fn decrement_target_count(&mut self) -> bool {
        if self.target_counter > 1 {
            self.target_counter -= 1;
            false
        } else {
            true
        }
    }
}
