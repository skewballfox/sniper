use std::borrow::Cow;

use crate::util::sniper_proto::{snippet_component::Component, SnippetComponent};
use nom::{self, bytes::streaming::take_until, IResult};

pub(crate) fn get_text<S>(snippet_string: &str) -> IResult<&str, Component> {
    take_until("$")(snippet_string)
}
