use chart_js_rs::{
    bar::Bar, scatter::Scatter, ChartOptions, Dataset, NoAnnotations, XYDataset, XYPoint,
};
use dominator::{self, events, html, Dom};
use futures_signals::{
    internal,
    signal::{Mutable, Signal, SignalExt},
};
use rand::Rng;
use std::rc::Rc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

fn random() -> Vec<(usize, usize)> {
    let rng = rand::thread_rng();

    let rnd_y = (0..20).map(|_| rng.clone().gen_range(0..100));
    let rnd_x = (0..20).map(|_| rng.clone().gen_range(0..10));
    rnd_x.zip(rnd_y).collect()
}

#[derive(Debug, Clone)]
pub struct Model {
    chart: Mutable<&'static str>,
    data: Mutable<Rc<Vec<(usize, usize)>>>,
    data_2: Mutable<Rc<Vec<(usize, usize)>>>,
}
impl Model {
    async fn init() -> Rc<Self> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        Rc::new(Model {
            chart: Mutable::new("chart_one"),
            data: Mutable::new(Rc::new(random())),
            data_2: Mutable::new(Rc::new(random())),
        })
    }

    fn chart_not_selected(self: Rc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c != chart)
    }

    fn show_charts(self: Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        internal::Map2::new(
            self.chart.signal(),
            internal::Map2::new(
                self.data.signal_cloned(),
                self.data_2.signal_cloned(),
                |data, data_2| (data.clone(), data_2.clone()),
            ),
            move |c, (data, data_2)| match c.to_string().as_str() {
                "chart_one" => Some(self.clone().show_chart_one(data.to_vec(), data_2.to_vec())),
                "chart_two" => Some(self.clone().show_chart_two(data.to_vec())),
                _ => None,
            },
        )
    }

    fn show_chart_one(
        self: Rc<Self>,
        data: Vec<(usize, usize)>,
        data_2: Vec<(usize, usize)>,
    ) -> Dom {
        // construct and render chart here
        let id = "chart_one";

        let chart = Scatter::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                datasets: Vec::from([
                    XYDataset {
                        data: Some(
                            data.iter()
                                .map(|d| XYPoint {
                                    // iterate over our data to construct a dataset
                                    x: Some(d.0.into()), // use .into() to convert to a NumberorDateString
                                    y: Some(d.1.into()),
                                })
                                .collect::<Vec<_>>(), // collect into a Vec<XYPoint>
                        ),
                        borderColor: Some("red".into()),
                        backgroundColor: Some("lightcoral".into()),
                        pointRadius: Some(4.into()),
                        label: Some("Dataset 1".into()),
                        ..Default::default() // always use `..Default::default()` to make sure this works in the future
                    },
                    XYDataset {
                        data: Some(
                            data_2
                                .iter()
                                .map(|d| XYPoint {
                                    // iterate over our data to construct a dataset
                                    x: Some(d.0.into()), // use .into() to convert to a NumberorDateString
                                    y: Some(d.1.into()),
                                })
                                .collect::<Vec<_>>(), // collect into a Vec<XYPoint>
                        ),
                        borderColor: Some("blue".into()),
                        backgroundColor: Some("lightskyblue".into()),
                        pointRadius: Some(4.into()),
                        label: Some("Dataset 2".into()),
                        ..Default::default() // always use `..Default::default()` to make sure this works in the future
                    },
                ]),
                ..Default::default()
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default() // always use `..Default::default()` to make sure this works in the future
            },
            id: id.into(),
            ..Default::default()
        };
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
            .prop("id", id)
            .style("height", "calc(100vh - 270px)")
            .after_inserted(move |_| {
                chart.to_chart().render_mutate(id) // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see chart_two and index.html
            })
        })
    }
    fn show_chart_two(self: Rc<Self>, data: Vec<(usize, usize)>) -> Dom {
        // construct and render chart here
        let id = "chart_two";

        let chart = Bar::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                labels: Some(
                    // use a range to give us our X axis labels
                    (0..data.len())
                        .into_iter()
                        .map(|d| (d + 1).into())
                        .collect(),
                ),
                datasets: Vec::from([XYDataset {
                    data: Some(
                        data.iter()
                            .enumerate()
                            .map(|(x, d)| XYPoint {
                                // iterate over our data to construct a dataset
                                x: Some((x + 1).into()), // use enumerate to give us our X axis point
                                y: Some(d.1.into()),
                            })
                            .collect::<Vec<_>>(), // collect into a Vec<XYPoint>
                    ),
                    backgroundColor: Some("palegreen".into()),
                    borderColor: Some("green".into()),
                    borderWidth: Some(2.into()),
                    label: Some("Dataset 1".into()),
                    yAxisID: Some("y".into()),
                    ..Default::default() // always use `..Default::default()` to make sure this works in the future
                }]),
            },
            r#type: "bar".into(),
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default() // always use `..Default::default()` to make sure this works in the future
            },
            id: id.into(),
        };
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
            .prop("id", id)
            .style("height", "calc(100vh - 270px)")
            .after_inserted(move |_| {
                chart.to_chart().render(id) // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see chart_two and index.html
            })
        })
    }

    fn render(self: Rc<Self>) -> Dom {
        html!("div", {
            .class("section")
            .child(
                html!("div", {
                    .class(["buttons", "has-addons"])
                    .child(
                        html!("button", {
                            .class(["button", "is-info"])
                            .text("Randomise")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().data.set(Rc::new(random())); // randomise the data on button click
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-primary"])
                            .class_signal("is-light", self.clone().chart_not_selected("chart_one"))
                            .text("Chart 1")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("chart_one"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-success"])
                            .class_signal("is-light", self.clone().chart_not_selected("chart_two"))
                            .text("Chart 2")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("chart_two"); // change which chart is in view
                                }
                            })
                        })
                    )
                })
            )
            .child(
                html!("div", {
                    .class("section")
                    .child_signal(self.show_charts()) // render charts here, signal allows us to change the chart, see the `Dominator` crate for more
                })
            )
        })
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    let app = Model::init().await;

    dominator::append_dom(&dominator::body(), Model::render(app));

    Ok(())
}
