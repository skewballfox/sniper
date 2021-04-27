use fancy_regex;

struct SnippetParser {
    base=fancy_regex::Regex;

}

impl SnippetParser {
    pub fn new()->{
        base=fancy_regex::Regex::new("\${(\d|SNIPPET").unwrap()
    }
}