use ansi_to_tui::IntoText;
use piechart::{Chart, Data};
use ratatui::{layout::Rect, Frame};
use std::io::Write;

#[derive(Debug)]
pub struct RatatuiChart {
    chart_output: String,
}

impl RatatuiChart {
    pub fn new(data: Vec<Data>) -> Self {
        let mut chart = RatatuiChart {
            chart_output: String::new(),
        };
        Chart::new()
            .radius(9)
            .aspect_ratio(4)
            .legend(false)
            .draw_into(&mut chart, &data)
            .unwrap();
        chart
    }
    pub fn draw_chart(&self, frame: &mut Frame, area: Rect) {
        let chart_text = self.chart_output.into_text().unwrap();
        frame.render_widget(chart_text, area);
    }
}

impl Write for RatatuiChart {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.chart_output.push_str(&String::from_utf8_lossy(buf));
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.chart_output = String::new();
        Ok(())
    }
}
