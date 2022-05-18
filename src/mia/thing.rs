use std::fmt;

struct Lamp {
    on: bool,
}

struct Dummy {
    chert: i32,
}

struct MultiSensor {
    temperature: i32,
    humidity: i32,
    light: i32,
}

enum Kind {
    Lamp(Lamp),
    Dummy(Dummy),
    MultiSensor(MultiSensor),
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lamp(_) => write!(f, "lamp"),
            Self::Dummy(_) => write!(f, "dummy"),
            Self::MultiSensor(_) => write!(f, "multisensor"),
        }
    }
}

struct Thing {
    kind: Kind,
    id: String,
}
