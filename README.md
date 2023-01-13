# JSON based Protocol Library for Things using MQTT
## Messages
The following messages are (or will be..) supported for their topics:

### RequestRegistration
```json
{
  "RequestRegistration": {
    "id": 1
  }
}
```

### SetName
```json
{
  "SetName": {
    "id": 1,
    "name": "new_thing" // default name
  }
}
```

### MeasurementTemperature
```json
{
  "MeasurementTemperature": {
    "id": 1771,
    "temperature": 19.319078
  }
}
```

### MeasurmentHumidity
```json
{
  "MeasurmentHumidity": {
    "id": 1771,
    "humidity": 53
  }
}
```

### LampState
```json
{
  "LampState": {
    "id": 1771,
    "state": true
  }
}
```

### LampRGB
```json
{
  "SetLamp": {
    "id": 1771,
    "r": 1,
    "g": 2,
    "b": 3,
  }
}
```

## Parser
Messages are used only if parsed. Each topic uses different or even the same messages.
### Parser `registry`
The following messages are parsed in the `registry` topic:
- RequestRegistration
- SetName


### Parser `thing_input`
The following messages are parsed in the `thing_input` topic:
- MeasurementTemperature
- MeasurmentHumidity
- LampState
- LampRGB

## Usage example
This example shows how the protocl my be used by the thing and cloud.

### Thing registration

Communication in topic `registry`.

 ![](topics/registry.svg)


### Receive thing values

Communication in topic `thing_input`.

 ![](topics/thing_input.svg)