use futures::{
    future::{self, Ready},
    prelude::*,
};
use tarpc::{
    client, context,
    server::{self, Incoming},
};

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SnippetInfo {
    pub name: String,
    pub description: String,
}
#[tarpc::service]
pub trait SniperService {
    //NOTE: may simplify the api once client is implemented, such as removing
    //get_triggers, and making triggers a return value for add/drop_target/lib

    /// add a target to the list of currently tracked targets
    async fn add_target(session_id: String, uri: String, language: String);

    /// drop a target,
    /// drop a snippet set if no longer required by any targets
    async fn drop_target(session_id: String, uri: String, language: String);

    ///add a snippet set to the given target
    //async fn target_add_libs(session_id: String, uri: String, libs: Vec<String>);

    ///drop a snippet set for a given target
    //async fn target_drop_libs(session_id: String, uri: String, libs: Vec<String>);

    /// get the triggers for the snippets associated with a given target
    async fn get_completions(session_id: String, uri: String, input: Vec<u8>) -> Vec<SnippetInfo>;

    /// get a snippet
    async fn get_snippet(
        session_id: String,
        uri: String,
        snippet_key: String,
    ) -> Option<Vec<String>>;
}
