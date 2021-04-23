
//good artist copy, great artist steal
//https://github.com/kak-lsp/kak-lsp/blob/master/src/util.rs#L15
pub fn temp_dir() -> path::PathBuf {
    let mut path = env::temp_dir();
    path.push("kak-lsp");
    let old_mask = unsafe { libc::umask(0) };
    // Ignoring possible error during $TMPDIR/kak-lsp creation to have a chance to restore umask.
    let _ = fs::DirBuilder::new()
        .recursive(true)
        .mode(0o1777)
        .create(&path);
    unsafe {
        libc::umask(old_mask);
    }
    path.push(whoami::username());
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(&path)
        .unwrap();
    path
}

//good artist copy, great artist steal
//https://github.com/kak-lsp/kak-lsp/blob/master/src/main.rs#L209
fn request(config: &Config) {
    let mut input = Vec::new();
    stdin()
        .read_to_end(&mut input)
        .expect("Failed to read stdin");
    let mut path = util::temp_dir();
    path.push(&config.server.session);
    if let Ok(mut stream) = UnixStream::connect(&path) {
        stream
            .write_all(&input)
            .expect("Failed to send stdin to server");
    } else {
        spin_up_server(&input);
    }
}

fn spin_up_server(input: &[u8]) {
    let args = env::args()
        .filter(|arg| arg != "--request")
        .collect::<Vec<_>>();
    let mut cmd = Command::new(&args[0]);
    let mut child = cmd
        .args(&args[1..])
        .args(&["--daemonize", "--initial-request"])
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to run server");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input)
        .expect("Failed to write initial request");
    child.wait().expect("Failed to daemonize server");
}