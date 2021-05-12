use sniper::Target::*;

use neon::prelude::*;
use neon::register_module;

use std::os::unix::net::UnixStream;


struct SniperNode {
    sniper: SniperServiceClient,
}
impl Finalize for SniperNode {}


//TODO: needs some kind of target blacklist for situation
//where target isn't viable


    //let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
    //Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))

async fn connect(mut cx: FunctionContext) -> JsResult<JsBox<SniperNode>>{
    Ok(cx.boxed(SniperNode{
        sniper:snipper_common::client::init_client()
    }))
}
async fn add_target(mut cx: FunctionContext){
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let session_id= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx);
    let uri= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    let language= cx.argument::<JsBox<JsString>>(3)?.value(&mut cx); 
    client.add_target(session_id, uri, language).await
}

async fn drop_target(mut cx: FunctionContext){
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let session_id= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx);
    let uri= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    let language= cx.argument::<JsBox<JsString>>(3)?.value(&mut cx); 
    client.add_target(session_id, uri, language).await
}

async fn get_triggers(mut cx: FunctionContext) -> Vec<String>{
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let session_id= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx);
    let uri= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    
}

async fn get_snippet(mut cx: FunctionContext) -> JsResult<JsString> {
    ///once snippet has been matched, get snippet from sniper
    let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let language= cx.argument::<JsBox<JsString>>(1)?.value(&mut cx);
    let snippet_key= cx.argument::<JsBox<JsString>>(2)?.value(&mut cx);
    client.get_snippet(session_id, uri, language).await

}


register_module!(mut m, {
    //m.export_class("sniperNode",sniperNode)?;
    m.export_function("startSniper",start_sniper)?;
    Ok(())
});

