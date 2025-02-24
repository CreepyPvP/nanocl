use serde::{Serialize, Deserialize};

use crate::schema::namespaces;

/// Structure to create a namespace in database
#[derive(
  Debug, Clone, Serialize, Deserialize, Identifiable, Insertable, Queryable,
)]
#[diesel(primary_key(name))]
#[diesel(table_name = namespaces)]
#[serde(rename_all = "PascalCase")]
pub struct NamespaceDbModel {
  pub(crate) name: String,
  pub(crate) created_at: chrono::NaiveDateTime,
}
