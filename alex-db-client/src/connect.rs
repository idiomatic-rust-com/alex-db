use crate::{
    context::{Connection, Context},
    error::ClientError,
};
use reedline_repl_rs::clap::ArgMatches;
use std::io;
use uuid::Uuid;

pub async fn connect<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let address = args
        .get_one::<String>("address")
        .ok_or(ClientError::String("Address parse error"))?;

    let api_key = args.get_one::<String>("api_key");

    let api_key = match api_key {
        None => {
            println!("API key: ");
            let mut api_key = String::new();
            io::stdin()
                .read_line(&mut api_key)
                .map_err(|_| ClientError::String("API key parse error"))?;
            api_key = api_key.replace('\n', "");

            match Uuid::parse_str(&api_key) {
                Err(_) => None,
                Ok(api_key) => Some(api_key),
            }
        }
        Some(api_key) => match Uuid::parse_str(api_key) {
            Err(_) => None,
            Ok(api_key) => Some(api_key),
        },
    };

    for mut connection in context.connections.iter_mut() {
        connection.is_default = false;
    }

    let connection = Connection::new(address.to_owned(), api_key, true);

    context.connections.append(&mut vec![connection]);

    Ok(Some(format!("Connect {address}")))
}
