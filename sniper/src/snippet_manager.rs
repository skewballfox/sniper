use async_stream::stream;

use iter::empty;

use dashmap::{iter::Iter, DashMap, ReadOnlyView};

//use futures::lock::Mutex;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    vec,
};
use tokio::sync::mpsc::Sender;
use tonic::Status;

//use sniper_common::service::SnippetInfo;

use crate::{
    parser::Token,
    snippet::{Loader, Snippet, SnippetSet},
    target::TargetData,
    util::sniper_proto::{
        snippet_component::Component, Functor, SnippetComponent, SnippetInfo, Tabstop,
    },
};

use std::{borrow::Cow, collections::VecDeque, iter, pin::Pin, sync::Arc};

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

    pub(crate) fn fire(
        &self,
        language: String,
        snippet_name: String,
        tx: Sender<Result<SnippetComponent, Status>>,
    ) {
        let ammo = (*self.snippets).clone().into_read_only();
        discharge(&language, snippet_name, ammo, &tx);
        tx.closed();
    }
}

fn discharge(
    language: &String,
    snippet_name: String,
    ammo: ReadOnlyView<(String, String), Snippet>,
    tx: &Sender<Result<SnippetComponent, Status>>,
) {
    let snippet_key = &(language.into(), snippet_name.into());
    let mut tokens: Vec<Token> = Vec::new();
    if let Some(snippet) = ammo.get(snippet_key) {
        let content = Cow::from(snippet.body.clone());
        for i in 0..content.len() {
            tokens.append(&mut crate::parser::snippet_component(&content[i]));
        }
    } else {
        tx.closed();
        return;
    }

    for token in tokens {
        if let Some(snip_name) = strike(token, tx) {
            discharge(language, snip_name, ammo.clone(), tx)
        }
    }
}

fn strike(token: Token, tx: &Sender<Result<SnippetComponent, Status>>) -> Option<String> {
    match _chamber(token) {
        ChamberType::ReadyComponent(comp) => {
            tx.blocking_send(Ok(SnippetComponent {
                component: Some(comp),
            }));
        }
        ChamberType::Tab(tab_number, placeholders) => {
            let mut content = Vec::<SnippetComponent>::new();
            for placeholder in placeholders {
                let ph = _chamber(placeholder);
                //TODO: incomplete, come back and finish logic for tabstops
                if let ChamberType::ReadyComponent(ph) = ph {
                    content.push(SnippetComponent {
                        component: Some(ph),
                    });
                };
            }
            tx.blocking_send(Ok(SnippetComponent {
                component: Some(Component::Tabstop(Tabstop {
                    number: tab_number as i32,
                    content,
                })),
            }));
        }
        ChamberType::Snippet(snip_name) => return Some(snip_name),
    };
    None
}
enum ChamberType {
    ReadyComponent(Component),
    Tab(u32, Vec<Token>),
    Snippet(String),
}
fn _chamber(token: Token) -> ChamberType {
    match token {
        Token::TabstopToken(tab_number, optional_placeholders) => {
            if let Some(placeholders) = optional_placeholders {
                ChamberType::Tab(tab_number, placeholders)
            } else {
                ChamberType::ReadyComponent(Component::Tabstop(Tabstop {
                    number: tab_number as i32,
                    content: Vec::new(),
                }))
            }
        }
        Token::TextToken(txt) => ChamberType::ReadyComponent(Component::Text(txt)),

        Token::VariableToken(name, transform) => {
            ChamberType::ReadyComponent(Component::Var(Functor {
                name: name,
                transform: transform,
            }))
        }
        Token::SnippetToken(sub_snippet_name) => ChamberType::Snippet(sub_snippet_name),
    }
}
