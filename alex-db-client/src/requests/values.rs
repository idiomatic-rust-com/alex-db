use crate::{
    context::Context,
    error::{ClientError, ServerError},
};
use alex_db_lib::{
    db::{Direction, Sort},
    value_record::{
        Value, ValueAppend, ValueDecrement, ValueIncrement, ValuePopBack, ValuePopFront, ValuePost,
        ValuePrepend, ValuePut, ValueResponse,
    },
};
use reedline_repl_rs::clap::ArgMatches;
use reqwest::StatusCode;
use std::{collections::VecDeque, str::FromStr};

pub async fn append<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let value = args
        .get_one::<String>("value")
        .ok_or(ClientError::String("Value parse error"))?;

    let value = Value::Array(VecDeque::from([Value::from_str(value)?]));

    let value_append = ValueAppend { append: value };

    let url = format!("{}/values/{key}/append", connection.address);

    let mut request_builder = reqwest::Client::new().put(url).json(&value_append);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_response: Result<ValueResponse, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_response {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_response) => {
            let mut response = String::new();
            response.push_str("Value appended\n");
            response.push_str(&format!(
                "Key: {}\nValue: {:?}\n",
                value_response.key, value_response.value
            ));

            Ok(Some(response))
        }
    }
}

pub async fn create<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let value = args
        .get_one::<String>("value")
        .ok_or(ClientError::String("Value parse error"))?;

    let value = Value::from_str(value)?;

    let ttl = match args.get_one::<String>("ttl") {
        None => None,
        Some(ttl) => match ttl.parse::<i64>() {
            Err(_) => None,
            Ok(ttl) => Some(ttl),
        },
    };

    let value_post = ValuePost {
        key: key.to_string(),
        ttl,
        value,
    };

    let url = format!("{}/values", connection.address);

    let mut request_builder = reqwest::Client::new().post(url).json(&value_post);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_response: Result<ValueResponse, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_response {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_response) => {
            let mut response = String::new();
            response.push_str("Value created\n");
            response.push_str(&format!(
                "Key: {}\nValue: {:?}\n",
                value_response.key, value_response.value
            ));

            Ok(Some(response))
        }
    }
}

pub async fn decrement<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let decrement = match args.get_one::<String>("decrement") {
        None => None,
        Some(decrement) => match decrement.parse::<i64>() {
            Err(_) => None,
            Ok(decrement) => Some(decrement),
        },
    };

    let value_decrement = ValueDecrement { decrement };

    let url = format!("{}/values/{key}/decrement", connection.address);

    let mut request_builder = reqwest::Client::new().put(url).json(&value_decrement);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_response: Result<ValueResponse, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_response {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_response) => {
            let mut response = String::new();
            response.push_str("Value decremented\n");
            response.push_str(&format!(
                "Key: {}\nValue: {:?}\n",
                value_response.key, value_response.value
            ));

            Ok(Some(response))
        }
    }
}

pub async fn delete<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let url = format!("{}/values/{key}", connection.address);

    let mut request_builder = reqwest::Client::new().delete(url);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?;

    match request_response.status() {
        StatusCode::NO_CONTENT => Ok(Some("Value deleted\n".to_string())),
        _status_code => {
            let request_response = request_response.text().await?;

            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
    }
}

pub async fn increment<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let increment = match args.get_one::<String>("increment") {
        None => None,
        Some(increment) => match increment.parse::<i64>() {
            Err(_) => None,
            Ok(increment) => Some(increment),
        },
    };

    let value_increment = ValueIncrement { increment };

    let url = format!("{}/values/{key}/increment", connection.address);

    let mut request_builder = reqwest::Client::new().put(url).json(&value_increment);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_response: Result<ValueResponse, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_response {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_response) => {
            let mut response = String::new();
            response.push_str("Value incremented\n");
            response.push_str(&format!(
                "Key: {}\nValue: {:?}\n",
                value_response.key, value_response.value
            ));

            Ok(Some(response))
        }
    }
}

pub async fn list<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let sort = args
        .get_one::<String>("sort")
        .unwrap_or(&Sort::Key.into())
        .clone();

    let direction = args
        .get_one::<String>("direction")
        .unwrap_or(&Direction::Asc.into())
        .clone();

    let limit = match args.get_one::<String>("limit") {
        None => 100,
        Some(limit) => limit.parse::<usize>().unwrap_or(100),
    };

    let page = match args.get_one::<String>("page") {
        None => 1,
        Some(page) => page.parse::<usize>().unwrap_or(1),
    };

    let url = format!(
        "{}/values?sort={sort}&direction={direction}&page={page}&limit={limit}",
        connection.address
    );
    let mut request_builder = reqwest::Client::new().get(url);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_responses: Result<Vec<ValueResponse>, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_responses {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_responses) => {
            let mut response = String::new();
            response.push_str("Values list\n");
            for (index, value_response) in value_responses.iter().enumerate() {
                response.push_str(&format!(
                    "{}) Key: {}\nValue: {:?}\n",
                    index + 1,
                    value_response.key,
                    value_response.value
                ));
            }
            Ok(Some(response))
        }
    }
}

pub async fn pop_back<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let pop_back = match args.get_one::<String>("pop_back") {
        None => None,
        Some(pop_back) => match pop_back.parse::<usize>() {
            Err(_) => None,
            Ok(pop_back) => Some(pop_back),
        },
    };

    let value_pop_back = ValuePopBack { pop_back };

    let url = format!("{}/values/{key}/pop-back", connection.address);

    let mut request_builder = reqwest::Client::new().put(url).json(&value_pop_back);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let values: Result<Vec<Value>, serde_json::Error> = serde_json::from_str(&request_response);

    match values {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(values) => {
            let mut response = String::new();
            response.push_str("Value poped back\n");

            for (index, value) in values.iter().enumerate() {
                response.push_str(&format!("{}) Value: {:?}\n", index + 1, value));
            }

            Ok(Some(response))
        }
    }
}

pub async fn pop_front<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let pop_front = match args.get_one::<String>("pop_front") {
        None => None,
        Some(pop_front) => match pop_front.parse::<usize>() {
            Err(_) => None,
            Ok(pop_front) => Some(pop_front),
        },
    };

    let value_pop_front = ValuePopFront { pop_front };

    let url = format!("{}/values/{key}/pop-front", connection.address);

    let mut request_builder = reqwest::Client::new().put(url).json(&value_pop_front);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let values: Result<Vec<Value>, serde_json::Error> = serde_json::from_str(&request_response);

    match values {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(values) => {
            let mut response = String::new();
            response.push_str("Value poped front\n");

            for (index, value) in values.iter().enumerate() {
                response.push_str(&format!("{}) Value: {:?}\n", index + 1, value));
            }

            Ok(Some(response))
        }
    }
}

pub async fn prepend<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let value = args
        .get_one::<String>("value")
        .ok_or(ClientError::String("Value parse error"))?;

    let value = Value::Array(VecDeque::from([Value::from_str(value)?]));

    let value_prepend = ValuePrepend { prepend: value };

    let url = format!("{}/values/{key}/prepend", connection.address);

    let mut request_builder = reqwest::Client::new().put(url).json(&value_prepend);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_response: Result<ValueResponse, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_response {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_response) => {
            let mut response = String::new();
            response.push_str("Value prepended\n");
            response.push_str(&format!(
                "Key: {}\nValue: {:?}\n",
                value_response.key, value_response.value
            ));

            Ok(Some(response))
        }
    }
}

pub async fn read<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let url = format!("{}/values/{key}", connection.address);

    let mut request_builder = reqwest::Client::new().get(url);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_response: Result<ValueResponse, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_response {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_response) => {
            let mut response = String::new();
            response.push_str("Value readed\n");
            response.push_str(&format!(
                "Key: {}\nValue: {:?}\n",
                value_response.key, value_response.value
            ));

            Ok(Some(response))
        }
    }
}

pub async fn update<'a>(
    args: ArgMatches,
    context: &mut Context,
) -> Result<Option<String>, ClientError<'a>> {
    let connection = context
        .get_default_connection()
        .ok_or(ClientError::NoActiveConnection)?;

    let key = args
        .get_one::<String>("key")
        .ok_or(ClientError::String("Key parse error"))?;

    let value = args
        .get_one::<String>("value")
        .ok_or(ClientError::String("Value parse error"))?;

    let value = Value::from_str(value)?;

    let ttl = match args.get_one::<String>("ttl") {
        None => None,
        Some(ttl) => match ttl.parse::<i64>() {
            Err(_) => None,
            Ok(ttl) => Some(ttl),
        },
    };

    let value_put = ValuePut { ttl, value };

    let url = format!("{}/values/{key}", connection.address);

    let mut request_builder = reqwest::Client::new().put(url).json(&value_put);

    request_builder = match connection.api_key {
        None => request_builder,
        Some(api_key) => request_builder.header("X-Auth-Token", api_key.to_string()),
    };

    let request_response = request_builder.send().await?.text().await?;

    let value_response: Result<ValueResponse, serde_json::Error> =
        serde_json::from_str(&request_response);

    match value_response {
        Err(_) => {
            let server_error: Result<ServerError, serde_json::Error> =
                serde_json::from_str(&request_response);

            match server_error {
                Err(e) => Ok(Some(format!("Error: {e:?}"))),
                Ok(server_error) => Ok(Some(format!("Server error: {}", server_error.error))),
            }
        }
        Ok(value_response) => {
            let mut response = String::new();
            response.push_str("Value updated\n");
            response.push_str(&format!(
                "Key: {}\nValue: {:?}\n",
                value_response.key, value_response.value
            ));

            Ok(Some(response))
        }
    }
}
