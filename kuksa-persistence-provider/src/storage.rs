/********************************************************************************
* Copyright (c) 2024 Contributors to the Eclipse Foundation
*
* This program and the accompanying materials are made available under the
* terms of the Apache License 2.0 which is available at
* http://www.apache.org/licenses/LICENSE-2.0
*
* SPDX-License-Identifier: Apache-2.0
********************************************************************************/


pub mod filestorage;

pub use filestorage::FileStorage;
use tinyjson::JsonValue;

trait Storage {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(config: &JsonValue) -> Self;

    // Method signatures; these will return a string.
    fn get(&self, vsspath: &'static str) -> &'static str;
    
    fn set(&self, vsspath: &'static str, vssvalue: &'static str) -> Result<(), ()>;

}
