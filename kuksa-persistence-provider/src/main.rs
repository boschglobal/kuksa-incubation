/********************************************************************************
* Copyright (c) 2024 Contributors to the Eclipse Foundation
*
* This program and the accompanying materials are made available under the
* terms of the Apache License 2.0 which is available at
* http://www.apache.org/licenses/LICENSE-2.0
*
* SPDX-License-Identifier: Apache-2.0
********************************************************************************/

mod storage;

use std::{path::PathBuf};
use clap::Parser;
use tinyjson::JsonValue;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct CmdLine {
    /// JSON file containing the configuration
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}

fn main() {
    let args = CmdLine::parse();

    let config = args.config.unwrap_or_else(|| PathBuf::from("config.json"));
    
    //Check config exists
    if !config.exists() {
        eprintln!("Error: Can not find configuration at {}", config.display());
        std::process::exit(1);
    }

    println!("Reading configuration from: {}", config.display());
    // Reading configuration file into a string
    let config_str = std::fs::read_to_string(config).unwrap();

    println!("Configuration: {}", config_str);

    let parsed: JsonValue = config_str.parse().unwrap();
    println!("Parsed: {:?}", parsed);

    match parsed["state-storage"]["type"].get::<String>().unwrap().as_str() {
            "file" => {
                println!("File Storage");
                let mut _storage = storage::FileStorage::new(&parsed["state-storage"]);
            },
            _ => {
                eprintln!("Error: state storage type is invalid");
                std::process::exit(1);
            }
        }




    println!("Hello, world!");
}
