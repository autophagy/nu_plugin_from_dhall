use nu_plugin::{EngineInterface, EvaluatedCall, Plugin, PluginCommand, SimplePluginCommand};
use nu_protocol::{Category, Example, LabeledError, Signature, Type, Value};

pub struct DhallPlugin;
struct FromDhall;

impl Plugin for DhallPlugin {
    fn version(&self) -> String {
        env!("CARGO_PKG_VERSION").into()
    }

    fn commands(&self) -> Vec<Box<dyn PluginCommand<Plugin = Self>>> {
        vec![Box::new(FromDhall)]
    }
}

impl SimplePluginCommand for FromDhall {
    type Plugin = DhallPlugin;

    fn name(&self) -> &str {
        "from dhall"
    }

    fn signature(&self) -> Signature {
        Signature::build(PluginCommand::name(self))
            .allow_variants_without_examples(true)
            .input_output_types(vec![(Type::Binary, Type::Any)])
            .category(Category::Experimental)
            .filter()
    }

    fn description(&self) -> &str {
        "do the things with .dhall files"
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "Convert from a .dhall file into table",
                example: "open file.dhall --raw | from dhall",
                result: None,
            },
            Example {
                description: "Convert from a .dhall file into table",
                example: "open file.dhall",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &EngineInterface,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        let span = input.span();
        match input {
            Value::String { val, .. } => {
                match crate::from_dhall::from_dhall_string(val.clone(), span) {
                    Ok(e) => Ok(e),
                    Err(e) => Err(*e),
                }
            }
            v => Err(
                LabeledError::new(format!("requires string input, got {}", v.get_type()))
                    .with_label("Expected string from pipeline", call.head),
            ),
        }
    }
}
