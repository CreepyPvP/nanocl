use std::collections::HashMap;

use dialoguer::Confirm;
use dialoguer::theme::ColorfulTheme;
use tokio_util::codec;
use futures::StreamExt;
use ntex::http::StatusCode;
use bollard_next::service::ProgressDetail;
use indicatif::{ProgressStyle, ProgressBar, MultiProgress};

use nanocld_client::NanocldClient;
use nanocld_client::error::ApiError;

use crate::utils::print::*;
use crate::utils::math::calculate_percentage;
use crate::error::CliError;
use crate::models::{
  CargoImageOpts, CargoImageCommands, CargoImageRemoveOpts,
  CargoImageInspectOpts, CargoImageRow, CargoImageImportOpts,
};

async fn exec_cargo_image_ls(client: &NanocldClient) -> Result<(), CliError> {
  let items = client.list_cargo_image(None).await?;
  let rows = items
    .into_iter()
    .map(CargoImageRow::from)
    .collect::<Vec<CargoImageRow>>();
  print_table(rows);
  Ok(())
}

async fn exec_cargo_image_rm(
  client: &NanocldClient,
  options: &CargoImageRemoveOpts,
) -> Result<(), CliError> {
  if !options.skip_confirm {
    let result = Confirm::with_theme(&ColorfulTheme::default())
      .with_prompt(format!("Delete cargo images {}?", options.names.join(",")))
      .default(false)
      .interact();
    match result {
      Ok(true) => {}
      _ => {
        return Err(CliError::Custom {
          msg: "Aborted".into(),
        })
      }
    }
  }
  for name in &options.names {
    client.delete_cargo_image(name).await?;
  }
  Ok(())
}

fn update_progress(
  multiprogress: &MultiProgress,
  layers: &mut HashMap<String, ProgressBar>,
  id: &str,
  progress: &ProgressDetail,
) {
  let total: u64 = progress
    .total
    .unwrap_or_default()
    .try_into()
    .unwrap_or_default();
  let current: u64 = progress
    .current
    .unwrap_or_default()
    .try_into()
    .unwrap_or_default();
  if let Some(pg) = layers.get(id) {
    let percent = calculate_percentage(current, total);
    pg.set_position(percent);
  } else {
    let pg = ProgressBar::new(100);
    let style = ProgressStyle::with_template(
      "[{elapsed_precise}] [{bar:20.cyan/blue}] {pos:>7}% {msg}",
    )
    .unwrap()
    .progress_chars("=> ");
    pg.set_style(style);
    multiprogress.add(pg.to_owned());
    let percent = calculate_percentage(current, total);
    pg.set_position(percent);
    layers.insert(id.to_owned(), pg);
  }
}

pub(crate) async fn exec_cargo_image_create(
  client: &NanocldClient,
  name: &str,
) -> Result<(), CliError> {
  let mut stream = client.create_cargo_image(name).await?;
  let mut layers: HashMap<String, ProgressBar> = HashMap::new();
  let multiprogress = MultiProgress::new();
  multiprogress.set_move_cursor(false);
  while let Some(info) = stream.next().await {
    let info = info?;
    // If there is any error we stop the stream
    if let Some(error) = info.error {
      return Err(CliError::Api(ApiError {
        msg: format!(
          "Error while downloading image {} got error {}",
          &name, &error
        ),
        status: StatusCode::INTERNAL_SERVER_ERROR,
      }));
    }
    let status = info.status.unwrap_or_default();
    let id = info.id.unwrap_or_default();
    let progress = info.progress_detail.unwrap_or_default();
    match status.as_str() {
      "Pulling fs layer" => {
        update_progress(&multiprogress, &mut layers, &id, &progress);
      }
      "Downloading" => {
        update_progress(&multiprogress, &mut layers, &id, &progress);
      }
      "Download complete" => {
        if let Some(pg) = layers.get(&id) {
          pg.set_position(100);
        }
      }
      "Extracting" => {
        update_progress(&multiprogress, &mut layers, &id, &progress);
      }
      _ => {
        if layers.get(&id).is_none() {
          let _ = multiprogress.println(&status);
        }
      }
    };
    if let Some(pg) = layers.get(&id) {
      pg.set_message(format!("[{}] {}", &id, &status));
    }
  }
  Ok(())
}

async fn exec_cargo_image_inspect(
  client: &NanocldClient,
  opts: &CargoImageInspectOpts,
) -> Result<(), CliError> {
  let image = client.inspect_cargo_image(&opts.name).await?;
  print_yml(image)?;
  Ok(())
}

async fn exec_cargo_image_import(
  client: &NanocldClient,
  opts: &CargoImageImportOpts,
) -> Result<(), CliError> {
  let file = tokio::fs::File::open(&opts.file_path).await.unwrap();

  let byte_stream =
    codec::FramedRead::new(file, codec::BytesCodec::new()).map(|r| {
      let bytes = ntex::util::Bytes::from_iter(r?.freeze().to_vec());
      Ok::<ntex::util::Bytes, std::io::Error>(bytes)
    });

  client.import_cargo_image_from_tar(byte_stream).await?;
  Ok(())
}

pub async fn exec_cargo_image(
  client: &NanocldClient,
  cmd: &CargoImageOpts,
) -> Result<(), CliError> {
  match &cmd.commands {
    CargoImageCommands::List => exec_cargo_image_ls(client).await,
    CargoImageCommands::Inspect(opts) => {
      exec_cargo_image_inspect(client, opts).await
    }
    CargoImageCommands::Create(opts) => {
      exec_cargo_image_create(client, &opts.name).await
    }
    CargoImageCommands::Remove(args) => exec_cargo_image_rm(client, args).await,
    CargoImageCommands::Import(opts) => {
      exec_cargo_image_import(client, opts).await
    }
  }
}
