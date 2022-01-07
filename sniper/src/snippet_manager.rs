use iter::empty;

use dashmap::{iter::Iter, DashMap, ReadOnlyView};

//use futures::lock::Mutex;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    vec,
};
use regex::Regex;
//use sniper_common::service::SnippetInfo;

use crate::{
    snippet::{Loader, Snippet, SnippetSet},
    target::TargetData,
    util::sniper_proto::{SnippetComponent, SnippetInfo},
};

use std::{
    borrow::Cow,
    collections::VecDeque,
    iter,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct SnippetManager {
    pub(crate) snippets: Arc<DashMap<(String, String), Snippet>>,
    pub(crate) snippet_sets: Arc<DashMap<(String, String), SnippetSet>>,
}

impl SnippetManager {
    pub fn new(
        snippets: Arc<DashMap<(String, String), Snippet>>,
        snippet_sets: Arc<DashMap<(String, String), SnippetSet>>,
    ) -> Self {
        Self {
            snippets,
            snippet_sets,
        }
    }

    pub fn load(
        &mut self,
        language: &str,
        snip_set_name: &str,
        snippet_data: &str,
        target: &mut TargetData,
    ) {
        println!("loading started");
        let temp: Loader = serde_json::from_str(snippet_data.into()).unwrap();
        let mut snippet_set: Vec<String> = Vec::with_capacity(temp.snippets.len());

        for (snippet_key, snippet) in temp.snippets.iter() {
            target.triggers.insert(
                snippet.prefix.clone(),
                SnippetInfo {
                    name: snippet_key.to_owned(),
                    description: snippet.description.clone(),
                },
            );

            self.snippets.insert(
                (language.to_string(), snippet_key.to_owned()),
                snippet.to_owned(),
            );
            snippet_set.push(snippet_key.to_owned());
        }
        self.snippet_sets.insert(
            (language.into(), snip_set_name.into()),
            SnippetSet::new(snippet_set),
        );
        target.loaded_snippets.insert(snip_set_name.into());
    }

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
            .map(move |s| {
                (
                    self.snippets
                        .get(&(language.clone(), s.clone()))
                        .unwrap()
                        .prefix
                        .clone(),
                    SnippetInfo {
                        name: s.clone(),
                        description: self
                            .snippets
                            .get(&(language.clone(), s.clone()))
                            .unwrap()
                            .description
                            .clone(),
                    },
                )
            })
    }
    pub fn unload(&self, language: &str, snip_set_to_drop: &str) {
        for snippet_key in self
            .snippet_sets
            .get(&(language.into(), snip_set_to_drop.into()))
            .unwrap()
            .contents
            .iter()
        {
            self.snippets
                .remove(&(language.to_string(), snippet_key.to_string()));
        }
        self.snippet_sets
            .remove(&(language.into(), snip_set_to_drop.into()));
    }
    //TODO: implement increment/decrement after implementing TargetManager struct
    //use iterator to handle both managers at once?
    //pub fn increment(&self, )

    pub fn fire<S>(&self, language: S, snippet_name: S) -> impl Iterator<Item = SnippetComponent>
    where
        S: Into<String>,
    {
        let ammo = (*self.snippets).clone().into_read_only();
        //clone().into_read_only();
        chamber(language, snippet_name, ammo)
    }
}

fn chamber<S>(
    language: S,
    snippet_name: S,
    ammo: ReadOnlyView<(String, String), Snippet>,
) -> impl Iterator<Item = SnippetComponent>
where
    S: Into<String>,
{
    let snippet_key = &(language.into(), snippet_name.into());

    match ammo.get(snippet_key) {
        Some(snippet) => {
            snippet.body.into_iter().for_each(|line|{}

            )
            iter::empty::<SnippetComponent>()
        },
        None => iter::empty::<SnippetComponent>(),
    }
}
mod parser;
