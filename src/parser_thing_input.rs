use crate::{
    message_lamp_rgb::{self, LampRGB, LampRGBDescirption},
    message_lamp_state::{self, LampState, LampStateDescirption},
    message_measurement_humidity::{self, MeasurmentHumidity, MeasurmentHumidityDescirption},
    message_measurement_temperature::{
        self, MeasurementTemperature, MeasurementTemperatureDescirption,
    },
};

pub enum ThingInputType {
    MeasurementHumidityType(MeasurmentHumidityDescirption),
    MeasurementTemperatureType(MeasurementTemperatureDescirption),
    LampRGB(LampRGBDescirption),
    LampState(LampStateDescirption),
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

    if let Ok(it) = message_lamp_rgb::parse(payload) {
        return ThingInputType::LampRGB(LampRGBDescirption {
            lamp_rgb: LampRGB {
                id: it.lamp_rgb.id,
                r: it.lamp_rgb.r,
                g: it.lamp_rgb.g,
                b: it.lamp_rgb.b,
            },
        });
    }

    if let Ok(it) = message_lamp_state::parse(payload) {
        return ThingInputType::LampState(LampStateDescirption {
            lamp_state: LampState {
                id: it.lamp_state.id,
                state: it.lamp_state.state,
            },
        });
    }

    ThingInputType::None
}
