use nu_errors::ShellError;
use nu_protocol::{ReturnSuccess, ReturnValue, TaggedDictBuilder, UntaggedValue, Value};
use nu_source::Tag;

use serde_dhall::{NumKind, SimpleValue};

pub struct FromDhall {
    pub state: String,
    pub name_tag: Tag,
}

impl FromDhall {
    pub fn new() -> Self {
        Self {
            state: String::new(),
            name_tag: Tag::unknown(),
        }
    }
}

fn convert_dhall_value_to_nu_value(v: &SimpleValue, tag: impl Into<Tag>) -> Value {
    let tag = tag.into();
    let span = tag.span;
    match v {
        SimpleValue::Num(n) => match n {
            NumKind::Bool(b) => UntaggedValue::boolean(*b).into_value(tag),
            NumKind::Natural(n) => UntaggedValue::int(*n as i64).into_value(tag),
            NumKind::Integer(i) => UntaggedValue::int(*i).into_value(tag),
            NumKind::Double(d) => {
                UntaggedValue::decimal_from_float((*d).into(), span).into_value(tag)
            }
        },
        SimpleValue::Text(t) => UntaggedValue::string(t).into_value(tag),
        SimpleValue::Record(r) => {
            let mut collected = TaggedDictBuilder::new(&tag);

            for (k, v) in r.iter() {
                collected.insert_value(k.clone(), convert_dhall_value_to_nu_value(v, &tag));
            }

            collected.into_value()
        }
        SimpleValue::List(l) => {
            let r: Vec<Value> = l
                .iter()
                .map(|x| convert_dhall_value_to_nu_value(x, &tag))
                .collect();
            UntaggedValue::Table(r).into_value(tag)
        }
        SimpleValue::Optional(o) => match o {
            Some(x) => convert_dhall_value_to_nu_value(x, &tag),
            None => UntaggedValue::nothing().into_value(tag),
        },
        x => unimplemented!("Unsupported dhall case: {:?}", x), // TODO: Implement Union
    }
}

pub fn from_dhall_string_to_value(
    s: String,
    tag: impl Into<Tag>,
) -> Result<Vec<ReturnValue>, ShellError> {
    let tag = tag.into();
    let v: serde_dhall::SimpleValue = serde_dhall::from_str(&s).parse().map_err(|x| {
        ShellError::labeled_error(
            format!("Could not load dhall: {}", x),
            "could not load dhall from text",
            &tag,
        )
    })?;

    Ok(vec![ReturnSuccess::value(convert_dhall_value_to_nu_value(
        &v, tag,
    ))])
}
