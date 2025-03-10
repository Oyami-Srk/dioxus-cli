pub mod build;
pub mod cfg;
pub mod clean;
pub mod config;
pub mod create;
pub mod serve;
pub mod tool;
pub mod translate;

use crate::{
    cfg::{ConfigOptsBuild, ConfigOptsServe},
    custom_error,
    error::Result,
    gen_page, server, CrateConfig, Error,
};
use clap::{Parser, Subcommand};
use html_parser::{Dom, Element, Node};
use regex::Regex;
use serde::Deserialize;
use std::{
    fmt::{Display, Formatter},
    fs::{remove_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
    process::{Command, Stdio},
};

/// Build, bundle, & ship your Dioxus app.
#[derive(Parser)]
#[clap(name = "dioxus", version)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Commands,

    /// Enable verbose logging.
    #[clap(short)]
    pub v: bool,
}

#[derive(Parser)]
pub enum Commands {
    /// Build the Rust WASM app and all of its assets.
    Build(build::Build),
    /// Translate some source file into Dioxus code.
    Translate(translate::Translate),
    /// Build, watch & serve the Rust WASM app and all of its assets.
    Serve(serve::Serve),
    /// Init a new project for Dioxus.
    Create(create::Create),
    /// Clean output artifacts.
    Clean(clean::Clean),
    /// Dioxus config file controls.
    #[clap(subcommand)]
    Config(config::Config),
    /// Install  & Manage tools for Dioxus-cli.
    #[clap(subcommand)]
    Tool(tool::Tool),
}
