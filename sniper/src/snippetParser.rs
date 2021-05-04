use fancy_regex;

enum SnipComponent {
    tabstop{start_index: u32, end_index: u32},
    metatabstop(u32,u32),
    sub_snippet(u32,u32),
}
struct SnippetParser {
    base=fancy_regex::Regex;

}

impl SnippetParser {
    pub fn new()->{
        base=fancy_regex::Regex::new("\${(\d|SNIPPET").unwrap()
    }
}