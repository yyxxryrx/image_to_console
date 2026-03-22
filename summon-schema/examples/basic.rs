use serde_json::{Map, Value};
use summon_schema::{ToSchema, map};

/// Config
///
/// Config file content
struct Config {
    /// IP
    ip: String,
    /// Port
    port: u16,
}

impl ToSchema for Config {
    fn schema_type() -> Value {
        serde_json::json!("object")
    }

    fn schema() -> Map<String, Value> {
        let mut ip_schema = String::schema();
        ip_schema.extend(map! {
            "type": String::schema_type(),
            "title": "IP",
            "description": "IP",
        });
        let mut port_schema = u16::schema();
        port_schema.extend(map! {
            "type": u16::schema_type(),
            "title": "Port",
            "description": "Port",
        });
        map! {
            "title": "Config",
            "description": "Config\nConfig file content",
            "properties": {
                "ip": ip_schema,
                "port": port_schema,
            },
        }
    }
}

fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&Config::schema()).unwrap_or_default()
    )
}
