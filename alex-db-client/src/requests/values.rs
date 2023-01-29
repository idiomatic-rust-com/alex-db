use crate::{context::Context, error::ClientError};
use alex_db_lib::value_record::ValueResponse;
use reedline_repl_rs::clap::ArgMatches;

pub async fn list<'a>(
    _args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let url = format!("{}/values", connection.address);
    let mut request_builder = reqwest::Client::new().get(url);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let response = request_builder
        .send()
        .await?
        .json::<Vec<ValueResponse>>()
        .await?;

    Ok(Some(format!("Values list {response:?}")))
}
