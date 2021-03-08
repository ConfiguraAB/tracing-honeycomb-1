#![deny(
    warnings,
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs
)]

//! This crate provides:
//! - A tracing layer, `TelemetryLayer`, that can be used to publish trace data to honeycomb.io
//! - Utilities for implementing distributed tracing against the honeycomb.io backend
//!
//! As a tracing layer, `TelemetryLayer` can be composed with other layers to provide stdout logging, filtering, etc.

use eaze_tracing_distributed as tracing_distributed;

mod honeycomb;
mod span_id;
mod trace_id;
mod visitor;
mod sink;

pub use sink::{Sink, HoneycombIO, Stdout};
pub use honeycomb::HoneycombTelemetry;
pub use span_id::SpanId;
pub use trace_id::TraceId;
#[doc(no_inline)]
pub use tracing_distributed::{TelemetryLayer, TraceCtxError};
pub use visitor::HoneycombVisitor;

pub(crate) mod deterministic_sampler;

#[cfg(feature = "use_parking_lot")]
use parking_lot::Mutex;
#[cfg(not(feature = "use_parking_lot"))]
use std::sync::Mutex;

/// Register the current span as the local root of a distributed trace.
///
/// Specialized to the honeycomb.io-specific SpanId and TraceId provided by this crate.
pub fn register_dist_tracing_root(
    trace_id: TraceId,
    remote_parent_span: Option<SpanId>,
) -> Result<(), TraceCtxError> {
    tracing_distributed::register_dist_tracing_root(trace_id, remote_parent_span)
}

/// Retrieve the distributed trace context associated with the current span.
///
/// Returns the `TraceId`, if any, that the current span is associated with along with
/// the `SpanId` belonging to the current span.
///
/// Specialized to the honeycomb.io-specific SpanId and TraceId provided by this crate.
pub fn current_dist_trace_ctx() -> Result<(TraceId, SpanId), TraceCtxError> {
    tracing_distributed::current_dist_trace_ctx()
}

/// Construct a TelemetryLayer that does not publish telemetry to any backend.
///
/// Specialized to the honeycomb.io-specific SpanId and TraceId provided by this crate.
pub fn new_blackhole_telemetry_layer(
) -> TelemetryLayer<tracing_distributed::BlackholeTelemetry<SpanId, TraceId>, SpanId, TraceId> {
    TelemetryLayer::new(
        "honeycomb_blackhole_tracing_layer",
        tracing_distributed::BlackholeTelemetry::default(),
        move |tracing_id| SpanId { tracing_id },
    )
}

/// Construct a TelemetryLayer that publishes telemetry to honeycomb.io using the provided honeycomb config.
///
/// Specialized to the honeycomb.io-specific SpanId and TraceId provided by this crate.
pub fn new_honeycomb_telemetry_layer(
    service_name: &'static str,
    honeycomb_config: libhoney::Config,
) -> TelemetryLayer<HoneycombTelemetry<HoneycombIO>, SpanId, TraceId> {
    let sink = libhoney::init(honeycomb_config);
    // publishing requires &mut so just mutex-wrap it
    // FIXME: may not be performant, investigate options (eg mpsc)
    let sink = HoneycombIO(Mutex::new(sink));

    TelemetryLayer::new(
        service_name,
        HoneycombTelemetry::new(sink, None),
        move |tracing_id| SpanId { tracing_id },
    )
}

/// Construct a TelemetryLayer that publishes telemetry to honeycomb.io using the
/// provided honeycomb config, and sample rate. This function differs from
/// `new_honeycomb_telemetry_layer` and the `sample_rate` on the
/// `libhoney::Config` there in an important way. `libhoney` samples `Event`
/// data, which is individual spans on each trace. This means that using the
/// sampling logic in libhoney may result in missing event data or incomplete
/// traces. Calling this function provides trace-level sampling, meaning sampling
/// decisions are based on a modulo of the traceID, and events in a single trace
/// will not be sampled differently. If the trace is sampled, then all spans
/// under it will be sent to honeycomb. If a trace is not sampled, no spans or
/// events under it will be sent. When using this trace-level sampling, the
/// `sample_rate` parameter on the `libhoney::Config` should be set to 1, which
/// is the default.
///
/// Specialized to the honeycomb.io-specific SpanId and TraceId provided by this crate.
pub fn new_honeycomb_telemetry_layer_with_trace_sampling(
    service_name: &'static str,
    honeycomb_config: libhoney::Config,
    sample_rate: u32,
) -> TelemetryLayer<HoneycombTelemetry<HoneycombIO>, SpanId, TraceId> {
    let sink = libhoney::init(honeycomb_config);
    // publishing requires &mut so just mutex-wrap it
    // FIXME: may not be performant, investigate options (eg mpsc)
    let sink = HoneycombIO(Mutex::new(sink));

    TelemetryLayer::new(
        service_name,
        HoneycombTelemetry::new(sink, Some(sample_rate)),
        move |tracing_id| SpanId { tracing_id },
    )
}

/// Construct a TelemetryLayer that publishes telemetry to some sink.
///
/// Specialized to the honeycomb.io-specific SpanId and TraceId provided by this crate.
pub fn new_honeycomb_telemetry_layer_with_sink<S: Sink>(
    service_name: &'static str,
    sink: S,
) -> TelemetryLayer<HoneycombTelemetry<S>, SpanId, TraceId> {
    TelemetryLayer::new(
        service_name,
        HoneycombTelemetry::new(sink, None),
        move |tracing_id| SpanId { tracing_id },
    )
}

/// Construct a TelemetryLayer that publishes telemetry to some sink using the
/// provided honeycomb config, and sample rate. This function differs from
/// `new_honeycomb_telemetry_layer_with_sink` and the `sample_rate` on the
/// `libhoney::Config` there in an important way. `libhoney` samples `Event`
/// data, which is individual spans on each trace. This means that using the
/// sampling logic in libhoney may result in missing event data or incomplete
/// traces. Calling this function provides trace-level sampling, meaning sampling
/// decisions are based on a modulo of the traceID, and events in a single trace
/// will not be sampled differently. If the trace is sampled, then all spans
/// under it will be sent to honeycomb. If a trace is not sampled, no spans or
/// events under it will be sent.
///
/// Specialized to the honeycomb.io-specific SpanId and TraceId provided by this crate.
pub fn new_honeycomb_telemetry_layer_with_sink_and_trace_sampling<S: Sink>(
    service_name: &'static str,
    sink: S,
    sample_rate: u32,
) -> TelemetryLayer<HoneycombTelemetry<S>, SpanId, TraceId> {
    TelemetryLayer::new(
        service_name,
        HoneycombTelemetry::new(sink, Some(sample_rate)),
        move |tracing_id| SpanId { tracing_id },
    )
}