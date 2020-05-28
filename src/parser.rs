use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;
use serde_json::Value;

// Represents a deserialized JSON line of the log file
#[derive(Deserialize)]
pub struct LogEntry {
    r#type: String,

    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

impl LogEntry {
    // Recursively computes the size in bytes of a `sede_json` `Value`
    fn size_of(v: &Value) -> usize {
        match v {
            Value::Null => 0,
            Value::Bool(_) => 1,
            Value::Number(_) => 8,
            Value::String(x) => x.len(),
            Value::Array(x) => x.iter().map(|x| Self::size_of(x)).sum(),
            Value::Object(x) => x.values().map(|x| Self::size_of(x)).sum(),
        }
    }
}

// Type to store the counting <type, (count, size)>
pub struct LogCount(HashMap<String, (usize, usize)>);

impl LogCount {
    pub fn new() -> Self {
        LogCount(HashMap::new())
    }

    // Insert a LogEntry into our `LogCount` store
    pub fn insert(&mut self, log: LogEntry) {
        let count = self.0.entry(log.r#type).or_insert((0, 0));
        let size: usize = log.extra.values().map(|v| LogEntry::size_of(v)).sum();
        *count = (count.0 + 1, count.1 + size);
    }
}

// Implement `Display` for a nice table print of LogCount
impl fmt::Display for LogCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{0: <10} | {1: <10} | {2: <10}",
            "type", "count", "total bytes",
        )?;

        for (key, (count, bytes)) in &self.0 {
            writeln!(f, "{0: <10} | {1: <10} | {2: <10}", key, count, bytes)?;
        }

        Ok(())
    }
}
