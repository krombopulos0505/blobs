pub const SENSOR_COUNT: usize = 9;

#[derive(Copy, Clone, Debug)]
pub enum Sensor {
    Energy,
    SeeBlob,
    SeeFood,
    SeeWall,
    Brightness,
    LocX,
    LocY,
    GetSignal,
    Random,
}

impl Sensor {
    pub const ALL: [Self; SENSOR_COUNT] = [
        Sensor::Energy,
        Sensor::SeeBlob,
        Sensor::SeeFood,
        Sensor::SeeWall,
        Sensor::Brightness,
        Sensor::LocX,
        Sensor::LocY,
        Sensor::GetSignal,
        Sensor::Random,
    ];
}
