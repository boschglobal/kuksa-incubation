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
mod kuksaconnector;

use storage::Storage;

use std::collections::HashMap;
use std::{path::PathBuf,env};
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

#[tokio::main]
async fn main() {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    env_logger::init();
    

    let args = CmdLine::parse();

    let config = args.config.unwrap_or_else(|| PathBuf::from("config.json"));
    
    //Check config exists
    if !config.exists() {
        log::error!("Error: Can not find configuration at {}", config.display());
        std::process::exit(1);
    }

    log::info!("Reading configuration from: {}", config.display());
    // Reading configuration file into a string
    let config_str = std::fs::read_to_string(config).unwrap();

    log::debug!("Configuration file content: {}", config_str);

    let parsed: JsonValue = config_str.parse().unwrap();
    log::debug!("Parsed JSON data structure: {:?}", parsed);

    let mut _storage = match parsed["state-storage"]["type"].get::<String>().unwrap().as_str() {
            "file" => {
                storage::FileStorage::new(&parsed["state-storage"])
            },
            _ => {
                log::error!("Error: state storage type is invalid");
                std::process::exit(1);
            }
        };

        let mut restore_current_values: Vec<String> = vec![];
        let mut restore_actuation_values: Vec<String> = vec![];
        let mut watch_current_values: Vec<String> = vec![];
        let mut watch_actuation_values: Vec<String> = vec![];



        let section: Option<&HashMap<String, JsonValue>> = parsed["restore-only"].get();

        if section.is_some() {
            let elements: Option<&Vec<JsonValue>> = section.unwrap()["values"].get();
            if elements.is_some() {
                for path in elements.unwrap() {
                    restore_current_values.push(path.get::<String>().unwrap().to_string());
                }
            }
            let elements: Option<&Vec<JsonValue>> = section.unwrap()["actuators"].get();
            if elements.is_some() {
                for path in elements.unwrap() {
                    restore_actuation_values.push(path.get::<String>().unwrap().to_string());
                }
            }
        }

        let section: Option<&HashMap<String, JsonValue>> = parsed["restore-and-watch"].get();
        if section.is_some() {
            let elements: Option<&Vec<JsonValue>> = section.unwrap()["values"].get();
            if elements.is_some() {
                for path in elements.unwrap() {
                    restore_current_values.push(path.get::<String>().unwrap().to_string());
                    watch_current_values.push(path.get::<String>().unwrap().to_string());   
                }
            }
            let elements: Option<&Vec<JsonValue>> = section.unwrap()["actuators"].get();
            if elements.is_some() {
                for path in elements.unwrap() {
                    restore_actuation_values.push(path.get::<String>().unwrap().to_string());
                    watch_actuation_values.push(path.get::<String>().unwrap().to_string());
                }
            }
        }


        let kuksa_client = kuksaconnector::create_kuksa_client("grpc://127.0.01:55556");

        kuksaconnector::get_from_storage_and_set_values(&_storage, &kuksa_client, &restore_current_values).await;
        kuksaconnector::get_from_storage_and_set_actuations(&_storage, &kuksa_client, &restore_actuation_values).await;

        drop(restore_actuation_values);
        drop(restore_current_values);


}
