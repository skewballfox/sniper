
use std::sync::Arc;

use sniper_common::{client::{
    tokio, tarpc_context,//re-exports
    init_client}, //defined functions
    service::SniperServiceClient};//tarpc generated 
use neon::prelude::*;
use neon::register_module;
use neon::event::EventQueue;



struct SniperNode {
    service_client: SniperServiceClient,
    queue: Arc<EventQueue>
}
impl Finalize for SniperNode {}


//TODO: needs some kind of target blacklist for situation
//where target isn't viable


    //let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
    //Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))

pub fn connectSniper(mut cx: FunctionContext) -> JsResult<JsUndefined>{
    let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let queue = Arc::new(cx.queue());
    tokio::spawn(async move {
        let node= SniperNode{
            service_client:init_client().await,
            queue: queue.clone(),
        };
        queue.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![
                cx.null().upcast::<JsValue>(),
                cx.boxed(node).upcast(),
            ];
            callback.call(&mut cx, this, args)?;

            Ok(())
            
        });
    });
    Ok(cx.undefined())
}
/*
 fn add_target(mut cx: FunctionContext<'_>)->NeonResult<()>{
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let session_id:String= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx).clone();
    let uri:String= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    let language:String= cx.argument::<JsBox<JsString>>(3)?.value(&mut cx); 
    client.sniper.add_target(tarpc_context(),session_id, uri, language);
    Ok(())
}

fn drop_target(mut cx: FunctionContext)->NeonResult<()>{
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let session_id= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx);
    let uri= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    let language= cx.argument::<JsBox<JsString>>(3)?.value(&mut cx); 
    client.sniper.drop_target(tarpc_context(),session_id, uri, language);
    Ok(())
}

fn get_triggers(mut cx: FunctionContext) -> Vec<String>{
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let session_id= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx);
    let uri= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    let language= cx.argument::<JsBox<JsString>>(3)?.value(&mut cx);
    client.sniper.get_triggers(tarpc_context(),session_id, uri);

}

fn get_snippet(mut cx: FunctionContext) -> JsResult<JsString> {
    ///once snippet has been matched, get snippet from sniper
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let language= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx);
    let snippet_key= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    client.sniper.get_snippet(tarpc_context(),language,snippet_key)

}

*/
register_module!(mut m, {
    //m.export_class("sniperNode",sniperNode)?;
    m.export_function("connectSniper",connectSniper)?;
    Ok(())
});

