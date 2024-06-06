use plotly::common::{DashType, Marker, Mode, Title};
use plotly::{Plot, Scatter};

pub struct Plotter {
    plot: Plot,
}

impl Plotter {
    pub fn new() -> Self {
        Self { plot: Plot::new() }
    }

    pub fn line(&mut self, x_values: Vec<f64>, y_values: Vec<f64>, name: &str, is_dashed: bool) {
        let dash_type: DashType = if is_dashed {
            plotly::common::DashType::Dash
        } else {
            plotly::common::DashType::Solid
        };

        let trace = Scatter::new(x_values, y_values)
            .mode(Mode::Lines)
            .name(name)
            .line(plotly::common::Line::new().dash(dash_type));

        self.plot.add_trace(trace);
    }

    pub fn markers(&mut self, x_values: Vec<f64>, y_values: Vec<f64>, name: &str) {
        let markers = Scatter::new(x_values, y_values)
            .mode(Mode::Markers)
            .name(name)
            .marker(Marker::new().size(10));

        self.plot.add_trace(markers);
    }

    pub fn plot(&mut self, title: &str, filename: &str, show: bool, save: bool) {
        self.plot.set_layout(
            plotly::Layout::new()
                .width(1600)
                .height(900)
                .title(Title::from(title)),
        );

        if show {
            self.plot.show();
        }
        if save {
            self.plot.write_html(format!("{}.html", filename));
            self.plot.write_image(
                format!("{}.svg", filename),
                plotly::ImageFormat::SVG,
                1600,
                900,
                1.0,
            );
            // self.plot.write_image(format!("{}.pdf", filename), plotly::ImageFormat::PDF, 1600, 900, 1.0);
        }
    }
}
