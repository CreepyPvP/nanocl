use nanocl_stubs::resource::{
  Resource, ResourcePartial, ResourceConfig, ResourceQuery, ResourcePatch,
};

use super::http_client::NanocldClient;
use super::error::NanocldClientError;

impl NanocldClient {
  /// ## List resources
  ///
  /// List all existing resources
  ///
  /// ## Returns
  ///
  /// * [Result](Result) - The result of the operation
  ///   * [Ok](Vec<Resource>) - The resources
  ///   * [Err](NanocldClientError) - An error if the operation failed
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_with_unix_default();
  /// let namespaces = client.list_resource().await;
  /// ```
  ///
  pub async fn list_resource(
    &self,
    query: Option<ResourceQuery>,
  ) -> Result<Vec<Resource>, NanocldClientError> {
    let res = self
      .send_get(format!("/{}/resources", &self.version), query)
      .await?;

    Self::res_json(res).await
  }

  /// ## Create resource
  ///
  /// Create a new resource
  ///
  /// ## Arguments
  ///
  /// * [data](ResourcePartial) - The data of the resource to create
  ///
  /// ## Returns
  ///
  /// * [Result](Result) - The result of the operation
  ///   * [Ok](Resource) - The created resource
  ///   * [Err](NanocldClientError) - An error if the operation failed
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  /// use nanocl_stubs::resource::ResourceKind;
  ///
  /// let client = NanocldClient::connect_with_unix_default();
  /// let resource = client.create_resource(&ResourcePartial {
  ///   name: "my-resource".into(),
  ///   kind: String::from("Custom")s,
  ///   // Your config
  ///   config: serde_json::json!({}),
  /// }).await;
  /// ```
  ///
  pub async fn create_resource(
    &self,
    data: &ResourcePartial,
  ) -> Result<Resource, NanocldClientError> {
    let res = self
      .send_post(
        format!("/{}/resources", &self.version),
        Some(data),
        None::<String>,
      )
      .await?;

    Self::res_json(res).await
  }

  /// ## Inspect resource
  ///
  /// Inspect an existing resource
  ///
  /// ## Arguments
  ///
  /// * [key](str) - The key of the resource to inspect
  ///
  /// ## Returns
  ///
  /// * [Result](Result) - The result of the operation
  ///   * [Ok](Resource) - The inspected resource
  ///   * [Err](NanocldClientError) - An error if the operation failed
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_with_unix_default();
  /// let resource = client.inspect_resource("my-resource").await;
  /// ```
  ///
  pub async fn inspect_resource(
    &self,
    key: &str,
  ) -> Result<Resource, NanocldClientError> {
    let res = self
      .send_get(
        format!("/{}/resources/{key}", &self.version),
        None::<String>,
      )
      .await?;

    Self::res_json(res).await
  }

  /// ## Patch resource
  ///
  /// Patch an existing resource
  ///
  /// ## Arguments
  ///
  /// * [key](str) - The key of the resource to patch
  /// * [data](ResourcePartial) - The data to patch
  ///
  /// ## Returns
  ///
  /// * [Result](Result) - The result of the operation
  ///   * [Ok](Resource) - The patched resource
  ///   * [Err](NanocldClientError) - An error if the operation failed
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_with_unix_default();
  /// let resource = client.patch_resource("my-resource", serde_json::json!({})).await;
  /// ```
  ///
  pub async fn patch_resource(
    &self,
    key: &str,
    config: &ResourcePatch,
  ) -> Result<Resource, NanocldClientError> {
    let res = self
      .send_patch(
        format!("/{}/resources/{key}", &self.version),
        Some(config),
        None::<String>,
      )
      .await?;

    Self::res_json(res).await
  }

  /// ## Delete resource
  ///
  /// Delete an existing resource
  ///
  /// ## Arguments
  ///
  /// * [key](str) - The key of the resource to delete
  ///
  /// ## Returns
  ///
  /// * [Result](Result) - The result of the operation
  ///   * [Ok](Ok(())) - The operation succeeded
  ///   * [Err](NanocldClientError) - An error if the operation failed
  ///
  /// ## Example
  ///
  /// ```no_run,ignore
  /// use nanocld_client::NanocldClient;
  ///
  /// let client = NanocldClient::connect_with_unix_default();
  /// let resource = client.delete_resource("my-resource").await;
  /// ```
  ///
  pub async fn delete_resource(
    &self,
    key: &str,
  ) -> Result<(), NanocldClientError> {
    self
      .send_delete(
        format!("/{}/resources/{key}", &self.version),
        None::<String>,
      )
      .await?;

    Ok(())
  }

  pub async fn list_history_resource(
    &self,
    key: &str,
  ) -> Result<Vec<ResourceConfig>, NanocldClientError> {
    let res = self
      .send_get(
        format!("/{}/resources/{key}/histories", &self.version),
        None::<String>,
      )
      .await?;

    Self::res_json(res).await
  }

  pub async fn reset_resource(
    &self,
    name: &str,
    key: &str,
  ) -> Result<Resource, NanocldClientError> {
    let res = self
      .send_patch(
        format!("/{}/resources/{name}/histories/{key}/reset", &self.version),
        None::<String>,
        None::<String>,
      )
      .await?;

    Self::res_json(res).await
  }
}

#[cfg(test)]
mod tests {
  use nanocl_stubs::resource::{ResourcePartial, ResourcePatch};

  use super::*;

  #[ntex::test]
  async fn basic() {
    let client = NanocldClient::connect_with_unix_default();

    // list
    client.list_resource(None).await.unwrap();

    let config = serde_json::json!({
      "type": "object",
      "required": [
        "Watch"
      ],
      "properties": {
        "Watch": {
          "description": "Cargo to watch for changes",
          "type": "array",
          "items": {
            "type": "string"
          }
        }
      }
    });

    let resource = ResourcePartial {
      name: "test_resource2".to_owned(),
      version: "v0.0.1".to_owned(),
      kind: "Custom".to_owned(),
      config: config.clone(),
    };

    // create
    let resource = client.create_resource(&resource).await.unwrap();

    assert_eq!(resource.name, "test_resource2");
    assert_eq!(resource.kind, String::from("Custom"));

    // inspect
    let resource = client.inspect_resource("test_resource2").await.unwrap();
    assert_eq!(resource.name, "test_resource2");
    assert_eq!(resource.kind, String::from("Custom"));

    let new_resource = ResourcePatch {
      version: "v0.0.2".to_owned(),
      config: config.clone(),
    };

    // patch
    let resource = client
      .patch_resource("test_resource2", &new_resource)
      .await
      .unwrap();

    assert_eq!(resource.name, "test_resource2");
    assert_eq!(resource.kind, String::from("Custom"));

    // history
    let history = client
      .list_history_resource("test_resource2")
      .await
      .unwrap();
    assert!(history.len() > 1);

    // delete
    client.delete_resource("test_resource2").await.unwrap();
  }
}
