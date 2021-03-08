use std::collections::HashMap;
use libhoney::FieldHolder;

#[cfg(feature = "use_parking_lot")]
use parking_lot::Mutex;
#[cfg(not(feature = "use_parking_lot"))]
use std::sync::Mutex;

/// Reports data to some given backend
pub trait Sink {
    /// Reports data to the backend
    fn report_data(&self, data: HashMap<String, libhoney::Value>);
}

/// Sink that publishes events and spans to Honeycomb.io
#[derive(Debug)]
pub struct HoneycombIO(pub Mutex<libhoney::Client<libhoney::transmission::Transmission>>);
impl Sink for HoneycombIO {
    fn report_data(&self, data: HashMap<String, libhoney::Value>) {
        // succeed or die. failure is unrecoverable (mutex poisoned)
        #[cfg(not(feature = "use_parking_lot"))]
        let mut sink = self.0.lock().unwrap();
        #[cfg(feature = "use_parking_lot")]
        let mut sink = self.0.lock();

        let mut ev = sink.new_event();
        ev.add(data);
        let res = ev.send(&mut sink);
        if let Err(err) = res {
            // unable to report telemetry (buffer full) so log msg to stderr
            // TODO: figure out strategy for handling this (eg report data loss event)
            eprintln!("error sending event to honeycomb, {:?}", err);
        }
    }
}

/// Sink that publishes events and spans to stdout
#[derive(Debug, Clone, Copy)]
pub struct Stdout;
impl Sink for Stdout {
    fn report_data(&self, data: HashMap<String, libhoney::Value>) {
        if let Ok(data) = serde_json::to_string(&data) {
            println!("{}", data);
        }
    }
}