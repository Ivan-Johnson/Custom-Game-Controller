use evdev::EventSummary;

pub fn summary_to_text(summary: EventSummary) -> String {
	match summary {
		EventSummary::Synchronization(_, _, _) => "sync".to_string(),
		EventSummary::Key(_, code, value) => {
			format!("key {code:?} {}", if value == 0 { "up" } else { "down" })
		}
		EventSummary::AbsoluteAxis(_, axis, value) => format!("axis {axis:?} {value:?}"),
		EventSummary::ForceFeedback(ff, _, _) => format!("Force Feedback - {ff:?}"),
		_ => todo!(),
	}
}
