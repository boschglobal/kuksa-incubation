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

pub struct FileStorage {
    storagefile: String,
}

impl FileStorage {
    pub fn new(config: &JsonValue) -> Self {
        match config["path"].get::<String>()
        {
            Some(x) => {
                println!("File Storage path: {}", x);
                //FileStorage { storagefile: "Lol" }
                let path = x.clone();
                FileStorage { storagefile: path}
            }
            _ => {
                eprintln!("Error: file storage path is invalid");
                std::process::exit(1);
            }
        }
    }

    fn get(&self, vsspath: &'static str) -> &'static str {
        println!("Try getting VSS signal {}", vsspath);
        &"LOL"
    }

    fn set(&self, vsspath: &'static str, vssvalue: &'static str) -> Result<(), ()> {
        println!("Setting VSS signal {} to {}", vsspath, vssvalue);
        Ok(())
    }
}
