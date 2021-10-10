use std::{
    borrow::Cow,
    collections::VecDeque,
    iter,
    sync::{Arc, Mutex},
};

use super::SnippetManager;
use crate::snippet::{SnipComponent, SnippetBuildMetadata};
use regex::Regex;

use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    vec,
};

#[macro_use]
use lazy_static::lazy_static;
pub(crate) enum SnippetTabstop {
    Simple,
    Placeholder,
    Choice,
}
pub(crate) enum SnippetVariable {
    Foo,
    Bar,
}

pub(crate) enum SnippetNode {
    Text(String),
    SnippetTabstop,
    SnippetVariable,
    SubSnippet,
}

// fn tabstop(input: &str) -> IResult<&str, String> {
//     alt(())
// }

impl SnippetManager {

    /// this function is used to build the snippets in the order of requirements
    ///this strategy works so long as there are no mutual dependencies among children
    ///though eventually an actual method for dependency resolution will have to be implemented
    pub fn rebuild_snippets(&self, language: &str, snippet_name: String) {
        let mut snippet_stack = VecDeque::new();
        let mut build_stack = VecDeque::new();
        snippet_stack.push_back(snippet_name);
        println!("starting to parse");
        while let Some(snip_name) = snippet_stack.pop_front() {
            println!("starting parsing for {:?}", snip_name);

            let (snipbuild, sub_snips) = self.parse_snippet(language.into(), &snip_name);

            (0..sub_snips.len()).into_iter().for_each(|i| {
                if self
                    .snippets
                    .contains_key(&(language.into(), sub_snips[i].clone()))
                {
                    if self
                        .snippets
                        .get(&(language.into(), sub_snips[i].clone()))
                        .unwrap()
                        .requires_assembly
                    {
                        snippet_stack.push_back(sub_snips[i].clone());
                    }
                }
            });
            build_stack.push_back(snipbuild);
        }
        println!("finished parsing,starting build process");
        while let Some(build_data) = build_stack.pop_back() {
            self.build_snippet(language, build_data);
        }
    }
    
    fn parse_snippet(
        &self,
        language: &str,
        snippet_name: &str,
    ) -> (SnippetBuildMetadata, Vec<String>) {
        //NOTE: while having more than one mutable reference inside a dashmap can risk deadlocks when multithreading,
        //there is no risk associated with multiple immutable ones
        //therefor I'm splitting the snippet rebuild process into two parts: parse_snippet and build_snippet
        //parse_snippet will generate metadata for snippet builds(asynchronously)
        lazy_static! {
            static ref digit: Regex = Regex::new(r"[[0-9]&&[^a-zA-Z]]+").unwrap();
            //TODO: deal with escaped characters such as \$ in bash
            static ref modification_needed: Regex = Regex::new(r"\$(\{?\d+|\$)").unwrap();
            static ref snippet_finder: Regex = Regex::new("[[a-zA-Z0-9/]]+").unwrap();
            static ref snippet_args_finder: Regex = Regex::new(r"\(.*\)}").unwrap();
        }
        let snippet_key = &(language.into(), snippet_name.to_string());
        let borrowed_body = Cow::from(self.snippets.get(snippet_key).unwrap().body.clone());
        let mut build_data: Vec<Vec<SnipComponent>> = Vec::with_capacity(borrowed_body.len());
        (0..borrowed_body.len()).into_iter().for_each(|i| {
            build_data.push(Vec::new());
        });
        let build_data_guard: Arc<Mutex<Vec<Vec<SnipComponent>>>> =
            Arc::new(Mutex::new(build_data));
        let stack_guard = Arc::new(Mutex::new(Vec::new()));
        println!("\nborrowed_body {:?}\n", borrowed_body);
        (0..borrowed_body.len())
            .into_par_iter()
            .for_each(|(line_index)| {
                let line = &borrowed_body[line_index];
                let mut line_data = Vec::new();
                for sub_match in modification_needed.find_iter(line) {
                    println!("{:?}", &line[sub_match.start()..sub_match.end()]);
                    let lead_char = line[sub_match.start() + 1..sub_match.end()]
                        .chars()
                        .nth(0)
                        .unwrap();

                    match lead_char {
                        '$' => {
                            let indices = snippet_finder.find(&line[sub_match.end()..]).unwrap();
                            let sub_snippet_name = &line[sub_match.end() + indices.start()
                                ..sub_match.end() + indices.end()];
                            let mut snippet_stack = stack_guard.lock().unwrap();
                            snippet_stack.push(sub_snippet_name.to_string());

                            let sub_snippet_name = &line[sub_match.end() + indices.start()
                                ..sub_match.end() + indices.end()];
                            line_data.push(SnipComponent::sub_snippet {
                                start: sub_match.end() + indices.start(),
                                end: sub_match.end() + indices.end(),
                                name: sub_snippet_name.into(),
                            });
                        }
                        _ => {
                            //println!("found tabstop at {:?}",&line[sub_match.start()..sub_match.end()]);
                            let indices = digit
                                .find(&line[sub_match.start()..sub_match.end()])
                                .unwrap();
                            line_data.push(SnipComponent::tabstop {
                                start: sub_match.start() + indices.start(),
                                end: sub_match.start() + indices.end(),
                            });
                        }
                    }
                }
                println!("{:?}", line_data);
                if !line_data.is_empty() {
                    let mut build_data = build_data_guard.lock().unwrap();
                    build_data[line_index] = line_data;
                }
            });
        //println!("{:?}",build_data);

        let snippet_stack = stack_guard.lock().unwrap();
        let build_data = Arc::try_unwrap(build_data_guard)
            .unwrap()
            .into_inner()
            .unwrap();
        println!("{:?}", build_data);
        (
            SnippetBuildMetadata::new(snippet_name.to_string(), snippet_stack.len(), build_data),
            snippet_stack.clone(),
        )
    }

    fn build_snippet(&self, language: &str, build_data: SnippetBuildMetadata) {
        let snippet_key = &(language.to_string(), build_data.name.clone());

        let mut new_body: Vec<String> = Vec::with_capacity(build_data.body.len());
        let old_body = Cow::from(self.snippets.get(snippet_key).unwrap().body.clone());
        let zero: usize = 0;
        let mut sub_snippet = Vec::new();
        let mut tabstops = Vec::new();
        let mut offset = 0;
        let mut contains_raw_content = true;
        println!("{:#?}", build_data);
        (0..build_data.body.len()).for_each(|line_index| {
            if build_data.body[line_index].is_empty() {
                new_body.push(old_body[line_index].clone());
            } else {
                println!("{:?}", &build_data.body[line_index]);
                for component_index in 0..build_data.body[line_index].len() {
                    match &build_data.body[line_index][component_index] {
                        SnipComponent::tabstop { start, end } => {
                            tabstops.push((line_index, *start, *end));
                            offset += 1;
                        }

                        SnipComponent::sub_snippet { start, end, name } => {
                            sub_snippet = self
                                .snippets
                                .get(&(language.to_string(), name.clone()))
                                .unwrap()
                                .body
                                .clone();
                            let mut sub_stops = self
                                .snippets
                                .get(&(language.to_string(), name.clone()))
                                .unwrap()
                                .tabstops
                                .clone();
                            for stop_index in 0..sub_stops.len() {
                                let (ref mut sub_line, sub_start, sub_end) = sub_stops[stop_index];
                                let digit = (sub_snippet[*sub_line][sub_start..sub_end])
                                    .parse::<i32>()
                                    .unwrap();
                                sub_snippet[*sub_line].replace_range(
                                    sub_start..sub_end,
                                    &(digit + offset).to_string(),
                                );
                                *sub_line += line_index;
                            }
                            //if *start==zero && end==&old_body[line_index].len(){
                            new_body.append(&mut sub_snippet);
                            contains_raw_content = false;
                            //TODO: actually workout substring checking for raw content

                            //}
                            offset += sub_stops.len() as i32;
                            tabstops.append(&mut sub_stops)
                        }

                        SnipComponent::metatabstop(_, _) => {}
                    }
                }
                if contains_raw_content {
                    new_body.push(old_body[line_index].clone());
                }
                contains_raw_content = true;
            }
        });
        drop(old_body);
        println!("\n new body created: {:?}", new_body);
        println!("\n list of tabstops: {:?}", tabstops);
        //println!("\n old body{:?}",self.snippets.get(&(language.to_string(),build_data.name.clone())).unwrap().body);
        self.snippets.get_mut(snippet_key).unwrap().body = new_body;
        self.snippets.get_mut(snippet_key).unwrap().tabstops = tabstops;
        self.snippets
            .get_mut(snippet_key)
            .unwrap()
            .requires_assembly = false;
        println!("snippet modified");
    }
    
}
