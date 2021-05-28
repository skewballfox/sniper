use std::sync::Arc;

use dashmap::DashMap;
use futures::lock::Mutex;

use qp_trie::Trie;
use sniper_common::service::SniperService;
use tarpc::{
    context,
    server::{self, Channel, Incoming},
};
use tokio::sync::RwLock;

use crate::{config::SniperConfig, snippet, snippet_manager::SnippetManager, target::TargetData};

#[derive(Clone)]
pub(crate) struct SniperServer {
    pub(crate) config: Arc<SniperConfig>,
    pub(crate) targets: Arc<DashMap<(String, String), TargetData>>,
    pub(crate) snippet_manager: SnippetManager,
}

impl SniperServer {
    pub fn new(
        config: Arc<SniperConfig>,
        targets: Arc<DashMap<(String, String), TargetData>>,
        snippet_manager: SnippetManager,
    ) -> Self {
        Self {
            config,
            targets,
            snippet_manager,
        }
    }
}
#[tarpc::server]
impl SniperService for SniperServer {
    /// add a session to the list of currently tracked sessions
    async fn add_target(
        self,
        _: context::Context,
        session_id: String,
        uri: String,
        language: String,
    ) {
        println!("adding target: {:?},{:?},{:?}", session_id, uri, language);
        //let sniper=self.snip_lock.read().await;
        if self
            .targets
            .contains_key(&(session_id.clone(), uri.clone()))
        {
            println!("target already tracked");
            return;
        }

        println!("loaded vars");
        //let targets=&*self.targets;
        if self.config.languages.contains_key(&language) {
            println!("config contains language {:?}", language);
            let mut target_data = TargetData::new(&language);

            let mut snippet_manager = self.snippet_manager.clone();
            //get a list of sets that needed to be loaded into the snippet manager
            self.config.languages[&language]
                .base_snippets
                .iter()
                .for_each(|snippet_set| {
                    if !snippet_manager
                        .snippet_sets //check if the snippet set has already been loaded
                        .contains_key(&(language.clone(), snippet_set.to_string()))
                    {
                        let snippet_data = self.config.get_snippet_data(&language, &snippet_set);
                        snippet_manager.load(
                            &language,
                            &snippet_set.to_string(),
                            &snippet_data.to_string(),
                            &mut target_data,
                        );
                    } else {
                        target_data.triggers.extend(
                            snippet_manager.triggers(language.clone(), snippet_set.clone()),
                        );
                        target_data.loaded_snippets.insert(snippet_set.to_string());
                    }
                });

            &self
                .targets
                .insert((session_id.into(), uri.into()), target_data);
            //should only track a target if it is in a supported language
            //should have some way of mitigating request for adding nonviable targets
            //client side
        }
        println!("target_added")
    }

    /// drop a target,
    /// drop a snippet set if no longer required by any targets
    /// exit sniper if no targets left
    async fn drop_target(
        self,
        _: context::Context,
        session_id: String,
        uri: String,
        language: String,
    ) {
        let target_key = &(session_id.to_string(), uri.to_string());

        println!("dropping target: {:?}", target_key);

        if self.targets.contains_key(target_key) {
            //consider using drain filter in the future:
            //https://doc.rust-lang.org/std/collections/struct.HashSet.html#method.drain_filter
            if let Some(target_data) = self.targets.remove(&(session_id, uri)) {
                for snip_set in target_data.1.loaded_snippets.iter() {
                    let drop_snippets_flag = self
                        .snippet_manager
                        .snippet_sets
                        .get_mut(&(language.to_string(), snip_set.to_string()))
                        .unwrap()
                        .decrement_target_count();
                    if drop_snippets_flag {
                        self.snippet_manager.unload(&language, &snip_set)
                    }
                }
            }
        }
        if self.targets.is_empty() {
            println!("todo");
            //sys.exit(0);
        }
    }

    /*async fn target_add_libs(self,_:context::Context,session_id: String, uri: String, libs: Vec<String>) {
        todo!()
    }

    async fn target_drop_libs(self,_:context::Context,session_id: String, uri: String, libs: Vec<String>) {
        todo!()
    }
    */

    async fn get_completions(
        self,
        _: context::Context,
        session_id: String,
        uri: String,
        input: Vec<u8>,
    ) -> Vec<String> {
        println!("{:?}", String::from_utf8(input.clone()));
        let target_key = (session_id, uri);
        let completions: Vec<String> = match Arc::clone(&self.targets).entry(target_key) {
            dashmap::mapref::entry::Entry::Occupied(ref target) => target
                .get()
                .triggers
                .iter_prefix(&input)
                .map(|(_trig, snip)| snip.clone())
                .collect::<Vec<String>>(),
            dashmap::mapref::entry::Entry::Vacant(_) => Vec::new(),
        };
        completions
    }

    async fn get_snippet(
        self,
        _: context::Context,
        session_id: String,
        uri: String,
        snippet_name: String,
    ) -> Option<Vec<String>> {
        let language = self
            .targets
            .get(&(session_id.to_string(), uri.to_string()))
            .unwrap()
            .language
            .clone();
        let snippet_key = &(language.to_string(), snippet_name.to_string());

        let mut assembly_required = false;
        let mut not_found = false;
        let mut snippet_body = Vec::new();
        println!("{:?} requested", snippet_name);
        if self.snippet_manager.snippets.contains_key(snippet_key) {
            if self
                .snippet_manager
                .snippets
                .get(snippet_key)
                .unwrap()
                .requires_assembly
            {
                assembly_required = true;
            } else {
                snippet_body = self
                    .snippet_manager
                    .snippets
                    .get(snippet_key)
                    .unwrap()
                    .body
                    .clone()
            }
        } else {
            println!("snippet not found");
            println!("snippets: {:?}", self.snippet_manager.snippets);
            not_found = true;
        }

        if not_found {
            None
        } else {
            if assembly_required {
                //only acquire writelock when necessary

                self.snippet_manager
                    .rebuild_snippets(&language, snippet_name.into());
                snippet_body = self
                    .snippet_manager
                    .snippets
                    .get(snippet_key)
                    .unwrap()
                    .body
                    .clone();
            }
            Some(snippet_body)
        }
    }
}
