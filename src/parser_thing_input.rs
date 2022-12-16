use crate::{
    message_measurement_humidity::{self, MeasurmentHumidity, MeasurmentHumidityDescirption},
    message_measurement_temperature::{
        self, MeasurementTemperature, MeasurementTemperatureDescirption,
    },
};

pub enum ThingInputType {
    MeasurementHumidityType(MeasurmentHumidityDescirption),
    MeasurementTemperatureType(MeasurementTemperatureDescirption),
    None,
}

pub fn parse(payload: &str) -> ThingInputType {
    if let Ok(it) = message_measurement_humidity::parse(payload) {
        return ThingInputType::MeasurementHumidityType(MeasurmentHumidityDescirption {
            measurement_humidity: MeasurmentHumidity {
                id: it.measurement_humidity.id,
                humidity: it.measurement_humidity.humidity,
            },
        });
    }

    if let Ok(it) = message_measurement_temperature::parse(payload) {
        return ThingInputType::MeasurementTemperatureType(MeasurementTemperatureDescirption {
            measurement_temperature: MeasurementTemperature {
                id: it.measurement_temperature.id,
                temperature: it.measurement_temperature.temperature,
            },
        });
    }
    ThingInputType::None
}
