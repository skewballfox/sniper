
use futures::{
    future::{self, Ready},
    prelude::*,
};
use tarpc::{
    client, context,
    server::{self, Incoming},
};
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
    //async fn get_triggers(session_id: String, uri: String)-> Vec<String>;
    
    /// get a snippet
    async fn get_snippet(language: String, snippet_key: String) -> Vec<String>;
    
}