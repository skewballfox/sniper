/*
   This is the implementation for all server-side responses, generally hands
   off the necessary parameters to other structs that are handling some part
   of the overall state

*/
use dashmap::DashMap;
use tonic::codegen::futures_core;

use std::sync::Arc;
use std::thread;
use tokio::sync::mpsc::{self};
use tokio_stream::wrappers::ReceiverStream;

use tonic::{Request, Response, Status};

use crate::util::sniper_proto::{SnippetComponent, SnippetRequest};

use crate::{config::SniperConfig, snippet_manager::SnippetManager, target::TargetData};

use crate::util::sniper_proto::{
    sniper_server::Sniper as SniperService, CompletionsRequest, CompletionsResponse, SnippetInfo,
    TargetRequest, Void,
};
pub(crate) type Stream<T> =
    std::pin::Pin<Box<dyn futures_core::Stream<Item = std::result::Result<T, Status>> + Send>>;

pub type SniperResponse<T> = Result<Response<T>, Status>;
#[derive(Clone)]
pub(crate) struct Sniper {
    pub(crate) config: Arc<SniperConfig>,
    pub(crate) targets: Arc<DashMap<(String, String), TargetData>>,
    pub(crate) snippet_manager: SnippetManager,
}

impl Sniper {
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

#[tonic::async_trait]
impl SniperService for Sniper {
    /// add a session to the list of currently tracked sessions
    async fn add_target(&self, request: Request<TargetRequest>) -> Result<Response<Void>, Status> {
        let TargetRequest {
            session_id,
            uri,
            language,
        } = request.into_inner();

        tracing::info!("adding target: {:?},{:?},{:?}", session_id, uri, language);
        //let sniper=self.snip_lock.read().await;
        if self
            .targets
            .contains_key(&(session_id.clone(), uri.clone()))
        {
            tracing::info!("target already tracked");
            return Ok(Response::new(Void {}));
        }
        tracing::info!("loaded vars");
        //let targets=&*self.targets;
        if self.config.languages.contains_key(&language) {
            tracing::info!("config contains language {:?}", language);
            let mut target_data = TargetData::new(&language);

            let mut snippet_manager = self.snippet_manager.clone();
            //get a list of sets that needed to be loaded into the snippet manager
            self.config.languages[&language]
                .base_snippets
                .iter()
                .for_each(|snippet_set| {
                    //check if the base snippet set has already been loaded
                    if !snippet_manager
                        .snippet_sets
                        .contains_key(&(language.clone(), snippet_set.to_string()))
                    {
                        //if the snippet set is currently untracked, load the base set of snippets
                        //for the targets language
                        let snippet_data = self.config.get_snippet_data(&language, &snippet_set);
                        snippet_manager.load(
                            &language,
                            &snippet_set.to_string(),
                            &snippet_data.to_string(),
                            &mut target_data,
                        );
                    } else {
                        //TODO: currently triggers is handled entirely by the snippet manager
                        // so may want to remove triggers from target_data and remove this line
                        target_data.triggers.extend(
                            snippet_manager.triggers(language.clone(), snippet_set.clone()),
                        );
                        target_data.loaded_snippets.insert(snippet_set.to_string());
                    }
                });

            let _ = &self
                .targets
                .insert((session_id.into(), uri.into()), target_data);
            //should only track a target if it is in a supported language
            //should have some way of mitigating request for adding nonviable targets
            //client side
        }
        tracing::debug!("target_added");
        Ok(Response::new(Void {}))
    }

    /// drop a target,
    /// drop a snippet set if no longer required by any targets
    /// exit sniper if no targets left
    async fn drop_target(&self, request: Request<TargetRequest>) -> Result<Response<Void>, Status> {
        let TargetRequest {
            session_id,
            uri,
            language,
        } = request.into_inner();
        let target_key = &(session_id.to_string(), uri.to_string());

        tracing::info!("dropping target: {:?}", target_key);
        //make sure that the target is already being tracked
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
        Ok(Response::new(Void {}))
    }

    ///given the current input text, return a list of relevant completions
    async fn get_completions(
        &self,
        request: Request<CompletionsRequest>,
    ) -> Result<Response<CompletionsResponse>, Status> {
        let CompletionsRequest {
            session_id,
            uri,
            user_input: keyboard_input,
        } = request.into_inner();
        tracing::info!(
            "User Input: {:?}",
            String::from_utf8(keyboard_input.clone())
        );
        let target_key = (session_id, uri);
        let _snippet_manager = self.snippet_manager.clone();
        let completions: Vec<SnippetInfo> = match Arc::clone(&self.targets).entry(target_key) {
            dashmap::mapref::entry::Entry::Occupied(ref target) => target
                .get()
                .triggers
                .iter_prefix(&keyboard_input)
                .map(|(_trig, snip)| snip.clone())
                .collect::<Vec<SnippetInfo>>(),
            dashmap::mapref::entry::Entry::Vacant(_) => Vec::new(),
        };
        Ok(Response::new(CompletionsResponse { completions }))
    }

    // only removing this temporarily, will reimplement once the project compiles with get_snippet
    /*async fn get_completions_stream(
        &self,
        req: Request<Streaming<CompletionsRequest>>,
    ) -> Result<Response<Stream<CompletionsResponse>>, Status> {
        let mut stream = req.into_inner();

        if let Some(first_msg) = stream.message().await? {
            let single_message = stream::iter(vec![Ok(first_msg)]);
            let mut stream = single_message.chain(stream);

            let stream = try_stream! {
            let snippet_manager = self.snippet_manager.clone();
            while let Some(msg) = stream.try_next().await? {

                let CompletionsRequest{session_id,uri,user_input:keyboard_input}=msg;
                let target_key = (session_id, uri);
                tracing::debug!("{:?}", String::from_utf8(keyboard_input.clone()));


            let completions: Vec<SnippetInfo> = match Arc::clone(&self.targets).entry(target_key) {
                dashmap::mapref::entry::Entry::Occupied(ref target) => target
                    .get()
                    .triggers
                    .iter_prefix(&keyboard_input)
                    .map(|(_trig, snip)| snip.clone())
                    .collect::<Vec<SnippetInfo>>(),
                dashmap::mapref::entry::Entry::Vacant(_) => Vec::new(),
            };
            yield CompletionsResponse { completions: completions.into() };

            }};
            return Ok(Response::new(
                Box::pin(stream) as Stream<CompletionsResponse>
            ));
        } else {
            let stream = stream::empty();
            return Ok(Response::new(
                Box::pin(stream) as Stream<CompletionsResponse>
            ));
        }
    }*/

    type GetSnippetStream = ReceiverStream<Result<SnippetComponent, Status>>;

    //type GetCompletionsStreamStream = Response<Stream<CompletionsResponse>>;

    ///gets and builds a snippet one piece at a time
    async fn get_snippet(
        &self,
        request: Request<SnippetRequest>,
    ) -> SniperResponse<Self::GetSnippetStream> {
        let SnippetRequest {
            session_id,
            uri,
            snippet_name,
        } = request.into_inner();
        tracing::info!("requesting snippet: {:?}", snippet_name);
        let (tx, rx) = mpsc::channel(64);
        let language = self
            .targets
            .get(&(session_id.to_string(), uri.to_string()))
            .unwrap()
            .language
            .clone();

        let snippet_manager = self.snippet_manager.clone();

        thread::spawn(move || {
            snippet_manager.fire(language, snippet_name, tx);
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
