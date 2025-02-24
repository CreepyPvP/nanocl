use std::{fs, str::FromStr};

use crate::error::ErrorHint;

use nanocld_client::stubs::proxy::ProxyRule;

#[derive(Debug)]
pub(crate) enum NginxConfKind {
  Site,
  Stream,
}

impl std::fmt::Display for NginxConfKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Site => write!(f, "Site"),
      Self::Stream => write!(f, "Stream"),
    }
  }
}

impl From<ProxyRule> for NginxConfKind {
  fn from(rule: ProxyRule) -> Self {
    match rule {
      ProxyRule::Http(_) => Self::Site,
      ProxyRule::Stream(_) => Self::Stream,
    }
  }
}

impl FromStr for NginxConfKind {
  type Err = ErrorHint;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Site" => Ok(Self::Site),
      "Stream" => Ok(Self::Stream),
      "site" => Ok(Self::Site),
      "stream" => Ok(Self::Stream),
      _ => Err(ErrorHint::error(1, format!("Invalid NginxConfKind: {}", s))),
    }
  }
}

#[derive(Clone, Debug)]
pub(crate) struct Nginx {
  pub(crate) conf_dir: String,
}

impl Nginx {
  pub(crate) fn new(conf_dir: &str) -> Self {
    Self {
      conf_dir: conf_dir.to_owned(),
    }
  }

  #[inline]
  fn gen_conf_path(&self, name: &str, kind: &NginxConfKind) -> String {
    match kind {
      NginxConfKind::Site => {
        format!("{}/sites-enabled/{name}.conf", &self.conf_dir)
      }
      NginxConfKind::Stream => {
        format!("{}/streams-enabled/{name}.conf", &self.conf_dir)
      }
    }
  }

  #[inline]
  pub(crate) fn ensure(&self) -> Result<(), ErrorHint> {
    // Ensure sites-enabled directory exists
    let sites_enabled_dir = format!("{}/sites-enabled", self.conf_dir);
    fs::create_dir_all(&sites_enabled_dir).map_err(|err| {
      ErrorHint::error(
        2,
        format!(
          "Cannot create directory {sites_enabled_dir} got error : {err}",
        ),
      )
    })?;
    // Ensure streams-enabled directory exists
    let streams_enabled_dir = format!("{}/streams-enabled", self.conf_dir);
    fs::create_dir_all(&streams_enabled_dir).map_err(|err| {
      ErrorHint::error(
        2,
        format!(
          "Cannot create directory {streams_enabled_dir} got error : {err}",
        ),
      )
    })?;
    // Ensure conf.d directory exists
    let conf_d = format!("{}/conf.d", self.conf_dir);
    fs::create_dir_all(conf_d).map_err(|err| {
      ErrorHint::error(
        2,
        format!(
          "Cannot create directory {streams_enabled_dir} got error : {err}",
        ),
      )
    })?;
    Ok(())
  }

  pub(crate) fn write_default_conf(&self) -> Result<(), ErrorHint> {
    let default_conf = "server {
  listen 80 default_server;
  listen [::]:80 default_server ipv6only=on;
  server_name _ default_server;

  root /usr/share/nginx/html;
  try_files $uri $uri/ /index.html;
  error_page 502 /502.html;
  error_page 403 /403.html;
}"
    .to_string();

    let path = format!("{}/conf.d/default.conf", self.conf_dir);

    fs::write(&path, &default_conf).map_err(|err| {
      ErrorHint::error(
        1,
        format!("Unable to create {path} file got error: {err}"),
      )
    })?;

    log::debug!("Writing default file conf:\n {default_conf}");

    Ok(())
  }

  #[inline]
  pub(crate) fn write_conf_file(
    &self,
    name: &str,
    data: &str,
    kind: &NginxConfKind,
  ) -> Result<(), ErrorHint> {
    let path = self.gen_conf_path(name, kind);
    fs::write(&path, data).map_err(|err| {
      ErrorHint::error(
        1,
        format!("Unable to create new site file {path} got error: {err}"),
      )
    })?;
    Ok(())
  }

  #[inline]
  pub(crate) async fn delete_conf_file(
    &self,
    name: &str,
    kind: &NginxConfKind,
  ) -> Result<(), ErrorHint> {
    let path = self.gen_conf_path(name, kind);
    ntex::web::block(move || {
      fs::remove_file(&path).map_err(|err| {
        ErrorHint::warning(
          3,
          format!("Unable to delete site file {path} got error: {err}"),
        )
      })
    })
    .await
    .map_err(|err| match err {
      ntex::web::error::BlockingError::Error(err) => err,
      ntex::web::error::BlockingError::Canceled => {
        ErrorHint::warning(3, "Blocking task canceled".into())
      }
    })?;
    Ok(())
  }

  #[inline]
  pub(crate) fn clear_conf(&self) -> Result<(), ErrorHint> {
    let sites_enabled_dir = format!("{}/sites-enabled", self.conf_dir);
    fs::remove_dir_all(&sites_enabled_dir).map_err(|err| {
      ErrorHint::error(
        3,
        format!(
          "Cannot remove directory {sites_enabled_dir} got error : {err}",
        ),
      )
    })?;
    let streams_enabled_dir = format!("{}/streams-enabled", self.conf_dir);
    fs::remove_dir_all(&streams_enabled_dir).map_err(|err| {
      ErrorHint::error(
        3,
        format!(
          "Cannot remove directory {streams_enabled_dir} got error : {err}",
        ),
      )
    })?;
    fs::create_dir_all(&sites_enabled_dir).map_err(|err| {
      ErrorHint::error(
        3,
        format!(
          "Cannot create directory {sites_enabled_dir} got error : {err}",
        ),
      )
    })?;
    fs::create_dir_all(&streams_enabled_dir).map_err(|err| {
      ErrorHint::error(
        3,
        format!(
          "Cannot create directory {streams_enabled_dir} got error : {err}",
        ),
      )
    })?;
    Ok(())
  }
}

/// Create a new nginx instance
pub(crate) fn new(config_path: &str) -> Nginx {
  Nginx::new(config_path)
}
