//! Checking a record against the schema committed beside it.
//!
//! Deliberately small and hand-written rather than a JSON-Schema dependency.
//! What has to be true is that the committed corpus and the committed schema
//! cannot drift apart, and the three keywords this schema uses — `required`,
//! `properties[].type` and `enum` — are what that needs. A validator that
//! understood more of the vocabulary than the schema uses would be a dependency
//! carrying a promise nothing here makes.

use serde_json::Value;

/// Checks one record against one schema document.
///
/// # Errors
///
/// Returns the first disagreement as a sentence naming the field, because a
/// reader six months out needs to know *which* field, not that something failed.
pub fn against(schema: &Value, record: &Value) -> Result<(), String> {
    let Some(object) = record.as_object() else {
        return Err("the record is not a JSON object".to_owned());
    };

    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        for name in required {
            let Some(name) = name.as_str() else { continue };
            if !object.contains_key(name) {
                return Err(format!("`{name}` is required and absent"));
            }
        }
    }

    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return Ok(());
    };
    for (name, value) in object {
        let Some(rule) = properties.get(name) else {
            return Err(format!(
                "`{name}` is in the record and not in the schema; one of the two \
                 moved without the other"
            ));
        };
        if let Some(kinds) = rule.get("type") {
            let allowed: Vec<&str> = match kinds {
                Value::String(one) => vec![one.as_str()],
                Value::Array(many) => many.iter().filter_map(Value::as_str).collect(),
                _ => Vec::new(),
            };
            if !allowed.is_empty() && !allowed.iter().any(|kind| matches(kind, value)) {
                return Err(format!(
                    "`{name}` is {}, and the schema allows {}",
                    kind_of(value),
                    allowed.join(" or ")
                ));
            }
        }
        if let Some(choices) = rule.get("enum").and_then(Value::as_array)
            && !choices.contains(value)
        {
            return Err(format!(
                "`{name}` is {value}, which the schema's enum excludes"
            ));
        }
    }
    Ok(())
}

fn matches(kind: &str, value: &Value) -> bool {
    match kind {
        "string" => value.is_string(),
        "integer" => value.is_i64() || value.is_u64(),
        "number" => value.is_number(),
        "boolean" => value.is_boolean(),
        "object" => value.is_object(),
        "array" => value.is_array(),
        "null" => value.is_null(),
        _ => true,
    }
}

fn kind_of(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(n) if n.is_f64() => "number",
        Value::Number(_) => "integer",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}
