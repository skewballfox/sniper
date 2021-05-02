fn main() {
    println!("Hello, world!");
    let session_id="12345";
    let test_uri="test.py";
    let lang="python";
    start_sniper(session_id,test_uri,lang);
    
    
   
}

fn start_sniper(session_id: S,test_uri: S, language: S) -> String where S: Into<String> {
    ///either connect to existing sniper session or start sniper session
    println!('todo');
    
    //good artist copy, great artist steal
    //https://github.com/kak-lsp/kak-lsp/blob/master/src/main.rs#L209
    if let Ok(mut stream) = UnixStream::connect(&path) {
        stream
            .write_all(&input)
            .expect("Failed to send stdin to server");
    } else {
        spin_up_server(&input);
    }
}
