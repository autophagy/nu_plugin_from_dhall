use nu_protocol::{LabeledError, Record, Span, Value};
use serde_dhall::{NumKind, SimpleValue};

fn convert_dhall_to_nu(v: &SimpleValue, span: Span) -> Value {
    match v {
        SimpleValue::Num(n) => match n {
            NumKind::Bool(b) => Value::bool(*b, span),
            NumKind::Natural(n) => Value::int(*n as i64, span),
            NumKind::Integer(i) => Value::int(*i, span),
            NumKind::Double(d) => Value::float((*d).into(), span),
        },
        SimpleValue::Text(t) => Value::string(t, span),
        SimpleValue::Record(r) => {
            let mut record = Record::new();

            for (k, v) in r.iter() {
                record.insert(k, convert_dhall_to_nu(v, span));
            }

            Value::record(record, span)
        }
        SimpleValue::List(l) => {
            let r: Vec<Value> = l.iter().map(|x| convert_dhall_to_nu(x, span)).collect();
            Value::list(r, span)
        }
        SimpleValue::Optional(o) => match o {
            Some(x) => convert_dhall_to_nu(x, span),
            None => Value::nothing(span),
        },
        SimpleValue::Union(k, v) => {
            let x = match v {
                Some(x) => convert_dhall_to_nu(x, span),
                None => Value::nothing(span),
            };
            let mut record = Record::new();
            record.insert(k, x);
            Value::record(record, span)
        }
    }
}

pub fn from_dhall_string(s: String, span: Span) -> Result<Value, Box<LabeledError>> {
    let v: serde_dhall::SimpleValue = serde_dhall::from_str(&s).parse().map_err(|x| {
        LabeledError::new(format!("Could not load dhall: {}", x)).with_label("oh no", span)
    })?;
    Ok(convert_dhall_to_nu(&v, span))
}
