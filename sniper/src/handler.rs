use std::{path::{Path, PathBuf}, sync::Arc};

use futures::lock::Mutex;
use service::SniperService;
use tarpc::{
    context,
    server::{self, Incoming, Channel},
    tokio_serde::formats::Json,
};
use tokio::net::{unix::SocketAddr};

use crate::sniper::Sniper;

#[derive(Clone)]
pub(crate) struct ConnectionHandler{
    //socket_address: PathBuf,
    pub(crate) sniper_mutex: Arc<Mutex<Sniper>>,
}


impl ConnectionHandler {
    pub fn new( sniper_mutex: Arc<Mutex<Sniper>>) -> Self { 
        Self { 
            //socket_address: PathBuf::from(client_socket_addr),
            sniper_mutex } 
    }
}
#[tarpc::server]
impl SniperService for ConnectionHandler {
    //type AddTargetFut = Type<>;
    async fn add_target(self,_:context::Context, session_id: String, uri: String, language: String) {
        let mut sniper=self.sniper_mutex.lock().await;
        //println!("what does this do");
        sniper.add_target(&session_id, &uri, &language);
    }

    async fn drop_target(self,_:context::Context,session_id: String, uri: String,language:String) {
        let mut sniper=self.sniper_mutex.lock().await;
        sniper.drop_target(&session_id, &uri,&language);
    }

    /*async fn target_add_libs(self,_:context::Context,session_id: String, uri: String, libs: Vec<String>) {
        todo!()
    }

    async fn target_drop_libs(self,_:context::Context,session_id: String, uri: String, libs: Vec<String>) {
        todo!()
    }
    */
    async fn get_triggers(self,_:context::Context,session_id: String, uri: String)-> Vec<String> {
        let sniper=self.sniper_mutex.lock().await;
        sniper.get_triggers( &session_id, &uri)
    }
    
    async fn get_snippet(self,_:context::Context,language: String, snippet_key: String) -> Vec<String> {
        let mut sniper=self.sniper_mutex.lock().await;
        sniper.snipe( &language, &snippet_key).unwrap()
    }
}