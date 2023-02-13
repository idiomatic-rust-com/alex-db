use crate::context::Context;
use reedline_repl_rs::{
    clap::{Arg, Command},
    {Repl, Result},
};

mod connect;
mod context;
mod error;
mod requests;

pub async fn run() -> Result<()> {
    let mut repl = Repl::new(Context::default())
        .with_name("AlexDB")
        .with_version("v0.1.0")
        .with_description("AlexDB client")
        .with_banner("Welcome to AlexDB client")
        .with_command_async(
            Command::new("append")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("value").required(true))
                .display_order(1)
                .about("Append value"),
            |args, context| Box::pin(requests::values::append(args, context)),
        )
        .with_command_async(
            Command::new("connect")
                .arg(Arg::new("address").required(true))
                .arg(Arg::new("api_key").required(false))
                .display_order(2)
                .about("Connect to database server"),
            |args, context| Box::pin(connect::connect(args, context)),
        )
        .with_command_async(
            Command::new("create")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("value").required(true))
                .arg(Arg::new("ttl").required(false))
                .display_order(3)
                .about("Create value"),
            |args, context| Box::pin(requests::values::create(args, context)),
        )
        .with_command_async(
            Command::new("decrement")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("decrement").required(false))
                .display_order(4)
                .about("Decrement value"),
            |args, context| Box::pin(requests::values::decrement(args, context)),
        )
        .with_command_async(
            Command::new("delete")
                .arg(Arg::new("key").required(true))
                .display_order(5)
                .about("Delete value"),
            |args, context| Box::pin(requests::values::delete(args, context)),
        )
        .with_command_async(
            Command::new("increment")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("increment").required(false))
                .display_order(6)
                .about("Increment value"),
            |args, context| Box::pin(requests::values::increment(args, context)),
        )
        .with_command_async(
            Command::new("list")
                .arg(Arg::new("sort").required(false))
                .arg(Arg::new("direction").required(false))
                .arg(Arg::new("limit").required(false))
                .arg(Arg::new("page").required(false))
                .display_order(7)
                .about("List values"),
            |args, context| Box::pin(requests::values::list(args, context)),
        )
        .with_command_async(
            Command::new("pop_back")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("pop_back").required(false))
                .display_order(8)
                .about("Pop back value"),
            |args, context| Box::pin(requests::values::pop_back(args, context)),
        )
        .with_command_async(
            Command::new("pop_front")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("pop_front").required(false))
                .display_order(9)
                .about("Pop front value"),
            |args, context| Box::pin(requests::values::pop_front(args, context)),
        )
        .with_command_async(
            Command::new("prepend")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("value").required(true))
                .display_order(10)
                .about("Prepend value"),
            |args, context| Box::pin(requests::values::prepend(args, context)),
        )
        .with_command_async(
            Command::new("read")
                .arg(Arg::new("key").required(true))
                .display_order(11)
                .about("Read value"),
            |args, context| Box::pin(requests::values::read(args, context)),
        )
        .with_command_async(
            Command::new("update")
                .arg(Arg::new("key").required(true))
                .arg(Arg::new("value").required(true))
                .arg(Arg::new("ttl").required(false))
                .display_order(12)
                .about("Update value"),
            |args, context| Box::pin(requests::values::update(args, context)),
        );

    repl.run_async().await
}
