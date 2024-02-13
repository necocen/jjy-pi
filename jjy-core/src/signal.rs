mod signal_type;
mod signal_value;

pub use signal_type::SignalType;
pub use signal_value::SignalValue;

#[derive(Debug, Clone, Copy)]
pub struct Signal {
    pub signal_type: SignalType,
    pub signal_value: SignalValue,
}
