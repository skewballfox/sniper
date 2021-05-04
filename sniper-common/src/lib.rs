
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
    
    async fn add_target(session_id: String, uri: String, language: String);
    
    async fn drop_target(session_id: String, uri: String);
    
    async fn target_add_libs(session_id: String, uri: String, libs: Vec<String>);
    
    async fn target_drop_libs(session_id: String, uri: String, libs: Vec<String>);
    
    async fn get_triggers(session_id: String, uri: String)-> Vec<String>;

    async fn get_snippet(language: String, snippet_key: String) -> Vec<String>;
    
}