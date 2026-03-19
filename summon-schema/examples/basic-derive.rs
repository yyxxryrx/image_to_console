use summon_schema::{Schema, ToSchema};

#[derive(Schema)]
#[serde(rename_all = "lowercase")]
enum UserType {
    Admin,
    Normal,
    Guest
}

#[derive(Schema)]
#[serde(rename_all = "kebab-case")]
struct UserInfo {
    /// Name of user
    username: String,
    /// User password
    password: String,
    /// Temp token
    temp_token: String,
    /// User type
    user_type: UserType
}

/// Config File
#[derive(Schema)]
struct Config {
    /// IP
    ip: Option<String>,
    /// Port
    port: u16,
    /// User 1 info
    user1: UserInfo,
    /// User 2 Info
    user2: UserInfo,
}

fn main() {
    println!(
        "{}",
        serde_json::to_string_pretty(&Config::schema()).unwrap()
    )
}
