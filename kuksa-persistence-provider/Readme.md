# KUKSA Persistence Provider

All data in KUKSA is ephemereal. However, in a car there is often data that does not change over the lifetime of a vehicle, and data where you want changes to be persisted over ignition cycles.

This provider can achieve this.

## Configuration: config,json

Main confiuguration is in config.json, and example may look like this

```json
{
    "restore-only": [
        "Vehicle.VehicleIdentification.VIN",
        "Vehicle.VehicleIdentification.VehicleInteriorColor"
    ],

    "restore-and-watch": [
        "Vehicle.Cabin.Infotainment.Media.Volume",
        "Vehicle.Cabin.Infotainment.HMI.TemperatureUnit"
    ],
    "state-storage": {
        "type": "file",
        "path": "statestore.json"
    }
}
```

## reestore-only section

These elements will be restored from the state store upon startup, but their values will not be watched and updated for changes.

## restore-and-watch

These elements will be restored from the state store upon startup. It is the intention to also monitor their state and update it in the state store. **This is currently not implemented**

## state-storage

Configures the state sotrage used to retrieve values. Currently supported: file

## File storage: statestore.json

This is a valid state store for the file storage.
*Note: ALl VALUES NEED TO BE STORED AS STRING*

```json

{
    "Vehicle.VehicleIdentification.VIN": {
        "value": "DEADBEEF"  
    },
    "Vehicle.VehicleIdentification.VehicleInteriorColor": {
        "value": "Black"
    }
}
```
