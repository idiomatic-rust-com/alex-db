#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserRecord {
    pub id: Uuid,
    pub api_key: Uuid,
    pub is_enabled: bool,
    pub password: String,
    pub roles: Vec<String>,
    pub username: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
