
pub(crate) enum SnipComponent {
    tabstop {start: u32, end: u32},
    metatabstop(u32,u32),
    sub_snippet{start: u32,end: u32, name: String},
}

pub(crate) struct BuiltSnippetMetadata {
    //subsnippet_tabstops: Hashmap<name:string,tabstops:vec<(u32,u32,u32)>?
    tabstops: Vec<(u32,u32,u32)>

}

pub(crate) struct SnippetBuildMetadata {
    name: String,
    sub_snippet_count: u32,
    tabstops: Vec<(u32,u32,u32)>,
    body: Vec<Vec<SnipComponent>>,
    sub_snippets:Vec<String>
}

impl SnippetBuildMetadata {
    pub(crate) fn new(name:String,sub_snip_count:u32,sub_snips: Vec<String>)->Self {
        Self {
            name: name,
            sub_snippet_count: sub_snip_count,
            tabstops: Vec::new(),
            body: Vec::new(),
            sub_snippets: sub_snips,
        }
    }
}
