use crate::FromDhall;

use nu_errors::ShellError;
use nu_plugin::Plugin;
use nu_protocol::{CallInfo, Primitive, ReturnValue, Signature, UntaggedValue, Value};
use nu_source::Tag;

impl Plugin for FromDhall {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("from dhall")
            .desc("Convert from a Dhall expression into a table.")
            .filter())
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        self.name_tag = call_info.name_tag;
        Ok(vec![])
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        match input {
            Value {
                value: UntaggedValue::Primitive(Primitive::String(s)),
                ..
            } => {
                self.state = s;
            }
            Value { tag, .. } => {
                return Err(ShellError::labeled_error_with_secondary(
                    "Expected string from pipeline",
                    "requires string input",
                    self.name_tag.clone(),
                    "value originates from here",
                    tag,
                ));
            }
        }
        Ok(vec![])
    }

    fn end_filter(&mut self) -> Result<Vec<ReturnValue>, ShellError> {
        crate::from_dhall::from_dhall_string_to_value(self.state.clone(), Tag::unknown())
    }
}
