use serde::{Deserialize, Serialize};
use valuable::{Fields, NamedField, NamedValues, StructDef, Structable, Valuable, Value, Visit};
use secrecy::SecretString;

#[derive(Debug, Clone, Deserialize, Serialize, Valuable)]
pub struct RepositoryConfig {
    #[serde(flatten)]
    pub backend: RepositoryBackend,
}

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
#[serde(tag = "type", content = "config")]
#[serde(rename_all = "lowercase")]
pub enum RepositoryBackend {
    Sqlite(SqliteConfig),
    Postgres(PostgresConfig)
}

#[derive(Debug, Clone, Serialize, Deserialize, Valuable)]
pub struct SqliteConfig {
    pub location: String,
    pub migration_path: String,
}

const VALUABLE_POSTGRES_CONFIG_FIELDS: &[NamedField<'static>] = &[
    NamedField::new("host"),
    NamedField::new("port"),
    NamedField::new("username"),
    NamedField::new("database_name"),
    NamedField::new("migration_path")
];


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub username: Option<String>,
    #[serde(skip_serializing)]
    pub password: Option<String>,
    pub database_name: String,
    pub migration_path: String,
}

impl Structable for PostgresConfig {
    fn definition(&self) -> StructDef<'_> {
        StructDef::new_static("PostgresConfig", Fields::Named(VALUABLE_POSTGRES_CONFIG_FIELDS))
    }
}

impl Valuable for PostgresConfig {
    fn as_value(&self) -> Value<'_> {
        Value::Structable(self)
    }

    fn visit(&self, v: &mut dyn Visit) {
        v.visit_named_fields(&NamedValues::new(
            VALUABLE_POSTGRES_CONFIG_FIELDS,
            &[
                Valuable::as_value(&self.host),
                Valuable::as_value(&self.port),
                Valuable::as_value(&self.username),
                Valuable::as_value(&self.database_name),
                Valuable::as_value(&self.migration_path)
            ],
        ));
    }
}

impl PostgresConfig {
    pub fn to_connection_string(&self) -> String {
        let mut connection_string = "postgresql://".to_string();
        if self.username.is_some() {
            connection_string += self.username.as_ref().unwrap().as_str();
        }
        if self.password.is_some() {
            connection_string += format!(":{}", self.password.as_ref().unwrap()).as_str();
        }
        if self.username.is_some() || self.password.is_some() {
            connection_string += "@";
        }
        connection_string += format!("{}:{}/{}", self.host, self.port, self.database_name).as_str();
        connection_string
    }
}

