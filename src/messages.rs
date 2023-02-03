use crate::{
    message_lamp_rgb::LampRGBDescirption, message_lamp_state::LampStateDescirption,
    message_measurement_humidity::MeasurmentHumidityDescirption,
    message_measurement_temperature::MeasurementTemperatureDescirption,
    message_request_registartion::RequestRegistrationDescirption,
    message_set_name::SetNameDescirption,
};

enum Messages {
    LampRGB(LampRGBDescirption),
    LampState(LampStateDescirption),
    MeasurmentHumidity(MeasurmentHumidityDescirption),
    MeasurmentTemperature(MeasurementTemperatureDescirption),
    RequestRegistration(RequestRegistrationDescirption),
    SetName(SetNameDescirption),
}
