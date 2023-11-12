mod command;
mod processor;
mod resource;
mod query;
use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let domain_name = &args[1];

    let _ = command::writer(domain_name);
    let _ = query::writer(domain_name);
    let _ = processor::writer(domain_name);
    let _ = resource::writer(domain_name);
    Ok(())
}
