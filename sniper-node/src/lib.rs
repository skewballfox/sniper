
use std::sync::Arc;

use sniper_common::{client::{
    tokio, tarpc_context,//re-exports
    init_client}, //defined functions
    service::SniperServiceClient};//tarpc generated 
use neon::prelude::*;
use neon::register_module;
use neon::event::EventQueue;
use once_cell::sync::OnceCell;


#[derive(Debug)]
struct SniperNode {
    client: SniperServiceClient,
    queue: Arc<EventQueue>,
    rt: tokio::runtime::Handle,
}
//impl Finalize for SniperNode {}

//static INSTANCE: OnceCell<SniperNode>=OnceCell::new();
/*
impl SniperNode {
    fn run()

    
}
*/

fn global_handler() -> &'static SniperNode {
    static INSTANCE: OnceCell<SniperNode>=OnceCell::new();
    match INSTANCE.get() {
        Some(it) => it,
        _ => unreachable!(),
    }
}

pub fn init(mut cx: FunctionContext)->JsResult<JsUndefined>{
    static INSTANCE: OnceCell<SniperNode>=OnceCell::new();
    println!("starting initialization");
    let queue = Arc::new(cx.queue());
        println!("initializing runtime");
        let rt=tokio::runtime::Runtime::new().unwrap();
        println!("starting initialization");
        let future=async move {
            let client=init_client().await;
            client
        };
        println!("connecting to server");
        
        let client=rt.block_on(future);
        println!("client connected");
        let rt_handle=rt.handle().clone();
        
    INSTANCE.set(SniperNode { client: client, queue: queue, rt: rt_handle }).unwrap();
    println!("instance set");
    Ok(cx.undefined())

}

//TODO: needs some kind of target blacklist for situation
//where target isn't viable


    //let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
    //Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))


fn add_target(mut cx: FunctionContext)->JsResult<JsUndefined>{
    //let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    let session_id=cx.argument::<JsString>(0).unwrap().value(&mut cx);
    let uri=cx.argument::<JsString>(1).unwrap().value(&mut cx);
    let language=cx.argument::<JsString>(2).unwrap().value(&mut cx);
    println!("failed here?");
    let handler=global_handler();
    println!("nope here");
    handler.rt.spawn(async move {
        println!("adding target");
        handler.client.add_target(tarpc_context(),session_id, uri, language);
        println!("target added");
    });
    Ok(cx.undefined())
    
}


fn drop_target(mut cx: FunctionContext)->NeonResult<()>{
    let session_id=cx.argument::<JsString>(0).unwrap().value(&mut cx);
    let uri=cx.argument::<JsString>(1).unwrap().value(&mut cx);
    let language=cx.argument::<JsString>(1).unwrap().value(&mut cx);
    let handler=global_handler();
    
    handler.client.drop_target(tarpc_context(),session_id, uri, language);
    Ok(())
}
/*
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
    m.export_function("init",init)?;
    m.export_function("add_target", add_target)?;
    Ok(())
});

