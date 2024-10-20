/********************************************************************************
* Copyright (c) 2024 Contributors to the Eclipse Foundation
*
* This program and the accompanying materials are made available under the
* terms of the Apache License 2.0 which is available at
* http://www.apache.org/licenses/LICENSE-2.0
*
* SPDX-License-Identifier: Apache-2.0
********************************************************************************/

use tinyjson::JsonValue;

use super::Storage;
use std::collections::HashMap;

use log;

pub struct FileStorage {
    storagefile: String,
    state: JsonValue,
}

impl Storage for FileStorage {
    fn new(config: &JsonValue) -> Self {
        match config["path"].get::<String>()
        {
            Some(x) => {
                log::info!("Initializing file storage on {}", x);
                let path = x.clone();
                println!("Reading storage from {}", path);
                let config_str = std::fs::read_to_string(&path).unwrap();
                let state = config_str.parse().unwrap();
                FileStorage { storagefile: path, state: state }
            }
            _ => {
                log::error!("Error: file storage path is invalid");
                std::process::exit(1);
            }
        }
    }

    fn get(&self, vsspath: &str) -> Option<&str> {
        log::debug!("Try getting VSS signal {}", vsspath);
        if !self.state.get::<HashMap<String, JsonValue>>().unwrap().contains_key(vsspath) {
            return None;
        }

        let entry: Option<&HashMap<String, JsonValue>> = self.state[vsspath].get();

        if entry.is_some() && entry.unwrap().contains_key("value") {
            let value = entry.unwrap()["value"].get::<String>();
            if value.is_some() {
                return Some(value.unwrap());
            }
        }
        return None;
    }

    fn set(&self, vsspath: &str, vssvalue: &'static str) -> Result<(), ()> {
        log::debug!("Setting VSS signal {} to {}", vsspath, vssvalue);
        Ok(())
    }

}

impl FileStorage {
    pub fn get_storagefile(&self) -> &String {
        &self.storagefile
    }
}