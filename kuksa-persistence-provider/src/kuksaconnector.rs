use crate::storage;

use regex;
use prost_types::Timestamp;

use std::time::SystemTime;
use std::collections::HashMap;

use kuksa::proto as proto;

//use kuksa::proto::v1::{datapoint::Value, DataType, Datapoint};

use std::sync::{Arc, Mutex};
use log;

#[derive(Debug)]
pub struct ParseError {}

pub fn create_kuksa_client(uri: &str) -> Arc<Mutex<kuksa::Client>> {
    log::info!("Creating Kuksa Databroker client for URI: {}", uri);
    let uri = kuksa::Uri::try_from(uri)
        .expect("Invalid URI  for Kuksa Databroker connection.");
    Arc::new(Mutex::new(kuksa::Client::new(uri)))
}

pub async fn get_from_storage_and_set(storage: &impl storage::Storage, kuksa_client: &Arc<Mutex<kuksa::Client>>, vsspath: &str) {
    log::debug!("Query storage for VSS signal: {}", vsspath);
    let value = match storage.get(vsspath) {
        Some(x) => x,
        None => {
            log::warn!("No value for VSS signal: {} stored", vsspath);
            return;
        }
    };
    
    /*
    let data_value = try_into_data_value(
        value,
        proto::v1::DataType::String).unwrap_or({log::error!("Error: Could not parse value {} as required datatype", value);return;});
 */
    let data_value = try_into_data_value(
    value,
    proto::v1::DataType::String).unwrap();
    
    //let ts = prost_types::protobuf::Timestamp::from(SystemTime::now());
    let ts = prost_types::Timestamp::from(SystemTime::now());

    //kuksa_client.lock().unwrap().set_current_values(datapoints)
    let datapoints = HashMap::from([(
        vsspath.to_string().clone(),
        proto::v1::Datapoint {
            timestamp: Some(ts),
            value: Some(data_value),
            },
    )]);

    match kuksa_client.lock().unwrap().set_current_values(datapoints).await {
        Ok(_) => {
            log::debug!("Succes setting {} to {}", vsspath, value);
        }
        Err(kuksa::Error::Status(status)) => {
            log::warn!("Error: Could not set value for VSS signal: {}, Status: {}", vsspath, &status);
        }
        Err(kuksa::Error::Connection(msg)) => {
            log::warn!("Connection Error: Could not set value for VSS signal: {}, Reason: {}", vsspath, &msg);
        }
        Err(kuksa::Error::Function(msg)) => {
            log::warn!("Error: Could not set value for VSS signal: {}, Errors: {msg:?}", vsspath);
        }
    };
    log::debug!("Got {} from storage  for {}" ,value, vsspath);
}


/* Donation from databroker-cli */
fn try_into_data_value(
    input: &str,
    data_type: proto::v1::DataType,
) -> Result<proto::v1::datapoint::Value, ParseError> {
    if input == "NotAvailable" {
        return Ok(proto::v1::datapoint::Value::String(input.to_string()));
    }

    match data_type {
        proto::v1::DataType::String => Ok(proto::v1::datapoint::Value::String(input.to_owned())),
        proto::v1::DataType::StringArray => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::StringArray(
                proto::v1::StringArray { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Boolean => match input.parse::<bool>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Bool(value)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::BooleanArray => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::BoolArray(
                proto::v1::BoolArray { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Int8 => match input.parse::<i8>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int32(value as i32)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Int8Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int32Array(
                proto::v1::Int32Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Int16 => match input.parse::<i16>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int32(value as i32)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Int16Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int32Array(
                proto::v1::Int32Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Int32 => match input.parse::<i32>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int32(value)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Int32Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int32Array(
                proto::v1::Int32Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Int64 => match input.parse::<i64>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int64(value)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Int64Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Int64Array(
                proto::v1::Int64Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Uint8 => match input.parse::<u8>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint32(value as u32)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Uint8Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint32Array(
                proto::v1::Uint32Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Uint16 => match input.parse::<u16>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint32(value as u32)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Uint16Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint32Array(
                proto::v1::Uint32Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Uint32 => match input.parse::<u32>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint32(value)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Uint32Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint32Array(
                proto::v1::Uint32Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Uint64 => match input.parse::<u64>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint64(value)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::Uint64Array => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::Uint64Array(
                proto::v1::Uint64Array { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Float => match input.parse::<f32>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Float(value)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::FloatArray => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::FloatArray(
                proto::v1::FloatArray { values: value },
            )),
            Err(err) => Err(err),
        },
        proto::v1::DataType::Double => match input.parse::<f64>() {
            Ok(value) => Ok(proto::v1::datapoint::Value::Double(value)),
            Err(_) => Err(ParseError {}),
        },
        proto::v1::DataType::DoubleArray => match get_array_from_input(input.to_owned()) {
            Ok(value) => Ok(proto::v1::datapoint::Value::DoubleArray(
                proto::v1::DoubleArray { values: value },
            )),
            Err(err) => Err(err),
        },
        _ => Err(ParseError {}),
    }
}




pub fn get_array_from_input<T: std::str::FromStr>(values: String) -> Result<Vec<T>, ParseError> {
    let raw_input = values
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .ok_or(ParseError {})?;

    let pattern = r#"(?:\\.|[^",])*"(?:\\.|[^"])*"|[^",]+"#;

    let regex = regex::Regex::new(pattern).unwrap();
    let inputs = regex.captures_iter(raw_input);

    let mut array: Vec<T> = vec![];
    for part in inputs {
        match part[0]
            .trim()
            .replace('\"', "")
            .replace('\\', "\"")
            .parse::<T>()
        {
            Ok(value) => array.push(value),
            Err(_) => return Err(ParseError {}),
        }
    }
    Ok(array)
}

