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
            Command::new("connect")
                .arg(Arg::new("address").required(true))
                .about("Connect to database server"),
            |args, context| Box::pin(connect::connect(args, context)),
        )
        .with_command_async(
            Command::new("list").about("List values"),
            |args, context| Box::pin(requests::values::list(args, context)),
        );

    repl.run_async().await
}
