
#[derive(Debug)]
pub(crate) enum SnipComponent {
    tabstop {start: usize, end: usize},
    metatabstop(u32,u32),
    sub_snippet{start: usize,end: usize, name: String},
}

pub(crate) struct BuiltSnippetMetadata {
    //subsnippet_tabstops: Hashmap<name:string,tabstops:vec<(u32,u32,u32)>?
    tabstops: Vec<(usize,usize,usize)>

}
#[derive(Debug)]
pub(crate) struct SnippetBuildMetadata {
    pub(crate)name: String,
    pub(crate)sub_snippet_count: usize,
    //pub(crate)tabstops: Vec<(usize,usize,usize)>,
    pub(crate)body: Vec<Vec<SnipComponent>>,
}

impl SnippetBuildMetadata {
    pub(crate) fn new(name: String,sub_snip_count:usize,body: Vec<Vec<SnipComponent>>)->Self {
        Self {
            name: name,
            sub_snippet_count: sub_snip_count,
            body: body,
        }
    }
}
