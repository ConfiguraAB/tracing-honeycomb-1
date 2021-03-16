use eaze_tracing_distributed as tracing_distributed;

use crate::reporter::Reporter;
use crate::visitor::{event_to_values, span_to_values, HoneycombVisitor};
use std::collections::HashMap;
use tracing_distributed::{Event, Span, Telemetry};

use crate::{SpanId, TraceId};

/// Telemetry capability that publishes Honeycomb events and spans to some backend
#[derive(Debug)]
pub struct HoneycombTelemetry<R> {
    reporter: R,
    sample_rate: Option<u32>,
}

impl<R: Reporter> HoneycombTelemetry<R> {
    pub(crate) fn new(reporter: R, sample_rate: Option<u32>) -> Self {
        HoneycombTelemetry {
            reporter,
            sample_rate,
        }
    }

    fn report_data(&self, data: HashMap<String, libhoney::Value>) {
        self.reporter.report_data(data);
    }

    fn should_report(&self, trace_id: &TraceId) -> bool {
        if let Some(sample_rate) = self.sample_rate {
            crate::deterministic_sampler::sample(sample_rate, trace_id)
        } else {
            false
        }
    }
}

impl<R: Reporter> Telemetry for HoneycombTelemetry<R> {
    type Visitor = HoneycombVisitor;
    type TraceId = TraceId;
    type SpanId = SpanId;

    fn mk_visitor(&self) -> Self::Visitor {
        Default::default()
    }

    fn report_span(&self, span: Span<Self::Visitor, Self::SpanId, Self::TraceId>) {
        if self.should_report(&span.trace_id) {
            let data = span_to_values(span);
            self.report_data(data);
        }
    }

    fn report_event(&self, event: Event<Self::Visitor, Self::SpanId, Self::TraceId>) {
        if self.should_report(&event.trace_id) {
            let data = event_to_values(event);
            self.report_data(data);
        }
    }
}
