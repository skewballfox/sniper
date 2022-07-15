/*
   As this is a snippet server, the majority of the state is stored here.
   handles loading and unloading snippets, getting a list of completions
   given the current user input, and handling things like turning a request
   into a snippet. Some pieces may be moved once things like "SnippetMode"
   and Functors are fully implemented

   All of the logic here is serial, primarily because it has to be. Triggers
   are stored in a trie, and parsing the body has to happen in order and relies
   on some recursive behavior since snippets can be composed of multiple snippets
*/
use dashmap::{DashMap, ReadOnlyView};

use rayon::iter::ParallelIterator;
use tokio::sync::mpsc::Sender;
use tonic::Status;

//use sniper_common::service::SnippetInfo;

use crate::{
    parser::Token,
    snippet::{Loader, RawSnippet, SnippetMetadata, SnippetSet},
    target::TargetData,
    util::sniper_proto::{
        snippet_component::Component, Functor, SnippetComponent, SnippetInfo, Tabstop,
    },
};

use std::{borrow::Cow, sync::Arc};
///The struct that stores all state related to the snippets themselves
#[derive(Debug, Clone)]
pub struct SnippetManager {
    /// The keys are (language, snippet_name), the value is the struct containing
    /// the deserialized snippet
    pub(crate) snippets: Arc<DashMap<(String, String), String>>,
    /// The keys are (language, set_name),  the set_name should correspond
    /// to the file name, or some way map to it elsewhere. The value is a
    /// struct with a vector of strings corresponding to the second half
    /// of the key for snippets
    pub(crate) snippet_sets: Arc<DashMap<(String, String), SnippetSet>>,
}

impl SnippetManager {
    pub fn new(
        snippets: Arc<DashMap<(String, String), String>>,
        snippet_sets: Arc<DashMap<(String, String), SnippetSet>>,
    ) -> Self {
        Self {
            snippets,
            snippet_sets,
        }
    }

    ///Once the client has requested a set of snippets this function adds the set of snippets into the manager
    pub(crate) fn load(
        &mut self,
        language: &str,
        snip_set_name: &str,
        snippet_data: &str,
        target: &mut TargetData,
    ) {
        println!("loading started");
        let temp: Loader = serde_json::from_str(snippet_data.into()).unwrap();
        let mut snippet_set: Vec<SnippetMetadata> = Vec::with_capacity(temp.snippets.len());
        //TODO: Consider moving to hashmap with drain
        //https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html#method.drain
        for (snippet_key, snippet) in temp.snippets.iter() {
            let RawSnippet {
                prefix,
                snippet_type,
                body,
                description,
                is_conditional,
                actions,
                requires_assembly,
                tabstops,
            } = snippet;

            target.triggers.insert(
                snippet.prefix.clone(),
                SnippetInfo {
                    name: snippet_key.to_owned(),
                    description: snippet.description.clone(),
                },
            );
            let contents = body.join("\n");
            self.snippets
                .insert((language.to_string(), snippet_key.to_owned()), contents);

            snippet_set.push(SnippetMetadata {
                prefix: prefix.to_owned(),
                description: description.to_owned(),
                name: snippet_key.to_owned(),
            });
        }
        self.snippet_sets.insert(
            (language.into(), snip_set_name.into()),
            SnippetSet::new(snippet_set),
        );
        target.loaded_snippets.insert(snip_set_name.into());
    }

    ///Returns a list of triggers associated with a snippet set, to be used when
    /// getting completions for a particular target
    pub fn triggers(
        &self,
        language: String,
        snippet_set: String,
    ) -> impl Iterator<Item = (Vec<u8>, SnippetInfo)> + '_ {
        self.snippet_sets
            .get(&(language.clone(), snippet_set.to_string()))
            .unwrap()
            .contents
            .clone() //TODO: figure out how to avoid cloning
            .into_iter()
            .map(move |s| s.to_snippet_info())
    }

    pub fn unload(&self, language: &str, snip_set_to_drop: &str) {
        for snippet_data in self
            .snippet_sets
            .get(&(language.into(), snip_set_to_drop.into()))
            .unwrap()
            .contents
            .iter()
        {
            self.snippets
                .remove(&(language.to_string(), snippet_data.get_name()));
        }
        self.snippet_sets
            .remove(&(language.into(), snip_set_to_drop.into()));
    }
    //TODO: implement increment/decrement after implementing TargetManager struct
    //use iterator to handle both managers at once?
    //pub fn increment(&self, )

    pub(crate) fn fire(
        &self,
        language: String,
        snippet_name: String,
        tx: Sender<Result<SnippetComponent, Status>>,
    ) {
        let ammo = (*self.snippets).clone().into_read_only();
        let mut offset = 0;
        chamber(&language, snippet_name, ammo, 0, &tx);
        tracing::debug!("closing component producer");
        tx.closed();
    }
}

fn chamber(
    language: &String,
    snippet_name: String,
    ammo: ReadOnlyView<(String, String), String>,
    mut tab_offset: i32,
    tx: &Sender<Result<SnippetComponent, Status>>,
) -> i32 {
    tracing::debug!("starting chamber for {:?}", snippet_name);
    let snippet_key = &(language.into(), snippet_name.into());
    let mut tokens: Vec<Token> = Vec::new();

    let mut tab_count = 0;

    if let Some(snippet) = ammo.get(snippet_key) {
        tracing::debug!("snippet found: {:#?}", snippet);
        let content = Cow::from(snippet.clone());
        let mut tmp: Vec<Token>;

        tmp = crate::parser::snippet_component(&content);
        if tmp.len() == 0 {
            tracing::error!("error encountered while parsing snippet: {:?}", snippet_key)
        }
        tracing::debug!("produced tokens {:?}", tmp);
        tokens.append(&mut tmp);
    } else {
        return 0;
    }

    for token in tokens {
        tracing::debug!("processing token {:#?}", token);
        match token {
            Token::ReadyComponent(component) => discharge(component, tx),
            Token::Tabstop(number, args) => {
                //handle args
                //TODO: rework when adding support for placeholders that aren't just raw text
                let content = args
                    .into_iter()
                    .map(|comp| SnippetComponent {
                        component: Some(comp),
                    })
                    .collect();
                discharge(
                    Component::Tabstop(Tabstop {
                        number: number as i32 + tab_offset,
                        content,
                    }),
                    tx,
                );
                tab_count += 1;
            }
            Token::Snippet(sub_snip) => {
                let mut sub_offset = tab_offset + tab_count;
                sub_offset = chamber(language, sub_snip, ammo.clone(), sub_offset, tx);

                if sub_offset != tab_offset + tab_count {
                    //if S is a snippet with 3 tabstops and V is a nested snippet with 2,
                    //where the layout is like S1,S2,V1,V2,S3
                    //in order for S3 to have the correct value (5), you must account
                    //for the offset of the nested snippet minus the current tab count
                    //for the parent snippet
                    tab_offset = sub_offset - tab_count;
                }
            }
        }
    }
    tab_offset + tab_count
}

fn discharge(component: Component, tx: &Sender<Result<SnippetComponent, Status>>) {
    let round = SnippetComponent {
        component: Some(component.clone()),
    };

    //just found out why get_snippet wasn't working
    //TODO: replace with something lock free as this is being called asynchronously
    tx.blocking_send(Ok(round))
        .expect(&format!("failed to send component {:?}", component));
}
