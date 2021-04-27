use sniper::Target::*;

use neon::prelude::*;
use neon::register_module;

use std::os::unix::net::UnixStream;


struct SniperNode {
    connection: sock,
}
impl Finalize for SniperNode {}


//TODO: needs some kind of target blacklist for situation
//where target isn't viable

fn start_sniper(mut cx: FunctionContext) -> JsResult<SniperNode> {
    ///either connect to existing sniper session or start sniper session
    println!('todo');
    let language = cx.argument::<JsString>(1).unwrap().value(&mut cx);
    //good artist copy, great artist steal
    //https://github.com/kak-lsp/kak-lsp/blob/master/src/main.rs#L209
    if let Ok(mut stream) = UnixStream::connect(&path) {
        stream
            .write_all(&input)
            .expect("Failed to send stdin to server");
    } else {
        spin_up_server(&input);
    }
};
    //let config_path = cx.argument::<JsString>(0)?.value(&mut cx);
    //Ok(cx.boxed(SniperNode { sniper:Sniper::new(&config_path) }))


fn get_snippet(mut cx: FunctionContext) -> JsResult<JsString> {
    ///once snippet has been matched, get snippet from sniper
    println!('todo');
}


register_module!(mut m, {
    //m.export_class("sniperNode",sniperNode)?;
    m.export_function("startSniper",start_sniper)?;
    Ok(())
});

