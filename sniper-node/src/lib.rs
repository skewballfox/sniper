
use std::{cell::RefCell, sync::{Arc, Mutex}};

use sniper_common::{client::{
    tokio, tarpc_context,//re-exports
    init_client}, //defined functions
    service::SniperServiceClient};//tarpc generated 
use neon::prelude::*;
use neon::register_module;
use neon::event::EventQueue;
use once_cell::sync::OnceCell;


#[derive(Debug)]
pub struct SniperNodeClient {
    client: Arc<Mutex<SniperServiceClient>>,
    queue: Arc<EventQueue>,
    rt: Box<tokio::runtime::Handle>,
}
impl Finalize for SniperNodeClient {}

//static INSTANCE: OnceCell<SniperNode>=OnceCell::new();
/*
impl SniperNode {
    fn run()

    
}
*/
static RT: OnceCell<Box<tokio::runtime::Handle>>=OnceCell::new();
static Q: OnceCell<Arc<Mutex<EventQueue>>>=OnceCell::new();
static  HANDLER: OnceCell<Arc<Mutex<SniperServiceClient>>>=OnceCell::new();


fn get_rt() -> &'static Box<tokio::runtime::Handle> {
    match RT.get() {
        Some(it) => it,
        _ => unreachable!(),
    }
}
fn get_queue() -> &'static Arc<Mutex<EventQueue>> {
    match Q.get() {
        Some(it) => it,
        _ => unreachable!(),
    }
}

fn get_client() -> &'static Arc<Mutex<SniperServiceClient>> {
    match HANDLER.get() {
        Some(it) => it,
        _ => unreachable!(),
    }
}


pub fn init(mut cx: FunctionContext)->JsResult<JsBox<SniperNodeClient>>{
    
    println!("starting initialization");
    let queue = Arc::new(EventQueue::new(&mut cx));
    println!("initializing runtime");
    let rt=tokio::runtime::Runtime::new().unwrap();
    
    println!("connecting to server");
    
    let client=rt.block_on(async move{init_client().await});
    //println!("{:#?}",client);
    let rt_handle=rt.handle().clone();
        
    //RT.set(Box::new(rt_handle)).unwrap();
    //Q.set(queue).unwrap();
    //HANDLER.set(Arc::new(Mutex::new(client))).unwrap();
    //println!("{:#?}",INSTANCE.get());
    Ok(cx.boxed(SniperNodeClient{client:Arc::new(Mutex::new(client)),queue,rt:Box::new(rt_handle)}))

}

//TODO: needs some kind of target blacklist for situation
//where target isn't viable
//TODO: find a way to cache client connection that doesn't lead to connection reset
//TODO: find a way to cache queue and RT that doesn't lead to runtime hang

    //let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
    //Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))


fn add_target(mut cx: FunctionContext)->JsResult<JsUndefined>{
    //let callback = cx.argument::<JsFunction>(0)?.root(&mut cx);
    //let sniper=cx.argument::<JsBox<SniperNodeClient>>(0).unwrap();
    let session_id=cx.argument::<JsString>(0).unwrap().value(&mut cx).clone();
    let uri=cx.argument::<JsString>(1).unwrap().value(&mut cx).clone();
    let language=cx.argument::<JsString>(2).unwrap().value(&mut cx).clone();
    println!("failed here?");
    
    println!("nope here");
    
    //let rt=sniper.rt.as_ref();
    //let client_lock=sniper.client.clone();
    //let rt=get_rt().as_ref();
    let rt=tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        println!("adding target");
        //let client=client_lock.lock().unwrap().clone();//init_client().await;
        let client=init_client().await;
        println!("client: {:?}",client);
        client.add_target(tarpc_context(),session_id, uri, language).await.unwrap();
        
    });
    println!("target added");
    Ok(cx.undefined())
    
}


fn drop_target(mut cx: FunctionContext)->JsResult<JsUndefined>{
    let session_id=cx.argument::<JsString>(0).unwrap().value(&mut cx);
    let uri=cx.argument::<JsString>(1).unwrap().value(&mut cx);
    let language=cx.argument::<JsString>(2).unwrap().value(&mut cx);
    //let handler=global_handler();
    println!("dropping target");
    //let client=get_client();
    //let rt=get_rt();
    let rt=tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let client=init_client().await;
        //let mut client=get_client().lock().unwrap();
        client.drop_target(tarpc_context(),session_id, uri, language).await.unwrap();
        //drop(client);
    });
    Ok(cx.undefined())
}

fn get_triggers(mut cx: FunctionContext) -> JsResult<JsUndefined>{

    let callback = cx.argument::<JsFunction>(2)?
        // Root the function so it can moved to the async block
        .root(&mut cx);
    let queue=cx.queue();
    //let queue=get_queue().lock().unwrap();
    
    let session_id=cx.argument::<JsString>(0)?.value(&mut cx).clone();
    let uri=cx.argument::<JsString>(1)?.value(&mut cx).clone();
    //let rt=get_rt();
    let rt=tokio::runtime::Runtime::new().unwrap();
    
    rt.block_on(async move {
        println!("trigger block started");
        //let client=get_client().lock().unwrap();
        let client=init_client().await;
        println!("got client");
        let triggers=client.get_triggers(tarpc_context(),session_id, uri).await.unwrap();
        println!("{:?}",triggers);
        
        //let queue=get_queue().lock().unwrap();
        queue.send(move |mut cx| {
            // "Un-root" the callback
            let callback = callback.into_inner(&mut cx);
            
            let jstrigs=JsArray::new(&mut cx,triggers.len() as u32);
            for (i,obj) in triggers.iter().enumerate(){
                let value=JsString::new(&mut cx,obj);
                jstrigs.set(&mut cx, i as u32, value).unwrap();
            }

            // Pieces of data required to invoke the callback
            let this = cx.undefined();
            let args = vec![
                // This is a Node style callback where the first argument is the error
                // Even though this code is infallible, using this format allows us
                // more easily promisify from JavaScript
                cx.null().upcast::<JsValue>(),
                jstrigs.upcast::<JsValue>(),
            ];
            
            callback.call(&mut cx, this, args)?;
            Ok(())
        });
    });
    Ok(cx.undefined())
    
}

fn get_snippet(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let callback = cx.argument::<JsFunction>(2)?
        // Root the function so it can moved to the async block
        .root(&mut cx);
    let queue=cx.queue();
    //let client = cx.argument::<JsBox<SniperNode>>(0)?;
    let language= cx.argument::<JsString>(0)?.value(&mut cx);
    let snippet_key= cx.argument::<JsString>(1)?.value(&mut cx);
    let rt=tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async move {
        let client=init_client().await;
        let snippet=client.get_snippet(tarpc_context(),language,snippet_key).await.unwrap().unwrap();
        println!("Snippet: {:?}",snippet);
        //let queue=get_queue().lock().unwrap();
        queue.send(move |mut cx| {
            // "Un-root" the callback
            let callback = callback.into_inner(&mut cx);
            
            let jssnippet=JsArray::new(&mut cx,snippet.len() as u32);
            for (i,obj) in snippet.iter().enumerate(){
                let value=JsString::new(&mut cx,obj);
                jssnippet.set(&mut cx, i as u32, value).unwrap();
            }

            // Pieces of data required to invoke the callback
            let this = cx.undefined();
            let args = vec![
                // This is a Node style callback where the first argument is the error
                // Even though this code is infallible, using this format allows us
                // more easily promisify from JavaScript
                cx.null().upcast::<JsValue>(),
                jssnippet.upcast::<JsValue>(),
            ];
            
            callback.call(&mut cx, this, args)?;
            Ok(())
        });

    });
    Ok(cx.undefined())

}


register_module!(mut m, {
    //m.export_class("sniperNode",sniperNode)?;
    m.export_function("init",init)?;
    m.export_function("add_target", add_target)?;
    m.export_function("drop_target", drop_target)?;
    m.export_function("get_triggers", get_triggers)?;
    m.export_function("get_snippet", get_snippet)?;
    Ok(())
});

