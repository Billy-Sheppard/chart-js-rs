use chart_js_rs::{
    bar::Bar, doughnut::Doughnut, pie::Pie, scatter::Scatter, ChartExt, ChartOptions, ChartScale,
    Dataset, DatasetDataExt, NoAnnotations, SinglePointDataset, XYDataset, XYPoint,
};
use dominator::{self, events, html, Dom};
use futures_signals::signal::{Mutable, MutableSignalCloned, Signal, SignalExt};
use rand::Rng;
use std::{
    collections::HashMap,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

fn random() -> Vec<usize> {
    let rng = rand::thread_rng();

    let rnd = (0..=20).map(|_| rng.clone().gen_range(1..50));

    rnd.collect()
}

#[derive(Debug, Clone)]
pub struct Model {
    tick: Mutable<bool>,
    chart: Mutable<&'static str>,
    x: Mutable<Rc<Vec<usize>>>,
    y1: Mutable<Rc<Vec<usize>>>,
    y2: Mutable<Rc<Vec<usize>>>,
}
impl Model {
    async fn init() -> Rc<Self> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        Rc::new(Model {
            tick: Mutable::default(),
            chart: Mutable::new("scatter"),
            x: Mutable::new(Rc::new((0..=20).collect())),
            y1: Mutable::new(Rc::new(random())),
            y2: Mutable::new(Rc::new(random())),
        })
    }

    fn chart_selected(self: Rc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c == chart)
    }
    fn chart_not_selected(self: Rc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c != chart)
    }

    fn show_charts(self: Rc<Self>) -> impl Signal<Item = Option<Dom>> {
        Mutable4::new(
            self.chart.clone(),
            self.x.clone(),
            self.y1.clone(),
            self.y2.clone(),
        )
        .map(move |(c, x, y1, y2)| match c.to_string().as_str() {
            "scatter" => Some(self.clone().show_scatter(
                x.as_slice(),
                y1.as_slice(),
                y2.as_slice(),
            )),
            "bar" => Some(self.clone().show_bar(y1.as_slice())),
            "donut" => Some(self.clone().show_donut()),
            "line" => Some(
                self.clone()
                    .show_line(x.as_slice(), y1.as_slice(), y2.as_slice()),
            ),
            _ => None,
        })
    }

    fn show_scatter(self: Rc<Self>, x: &[usize], y1: &[usize], y2: &[usize]) -> Dom {
        // construct and render chart here
        let id = "scatter";

        let chart = Scatter::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                datasets: Vec::from([
                    XYDataset {
                        data: x
                            .iter()
                            .zip(y1)
                            .map(|d| XYPoint {
                                // iterate over our data to construct a dataset
                                x: d.0.into(), // use .into() to convert to a NumberorDateString
                                y: d.1.into(),
                                ..Default::default() // always use `..Default::default()` to make sure this works in the future
                            })
                            .collect::<Vec<_>>()
                            .to_dataset_data(), // collect into a Vec<XYPoint>

                        borderColor: "red".into(),
                        backgroundColor: "lightcoral".into(),
                        pointRadius: 4.into(),
                        label: "Dataset 1".into(),
                        ..Default::default() // always use `..Default::default()` to make sure this works in the future
                    },
                    XYDataset {
                        data: x
                            .iter()
                            .zip(y2)
                            .map(|d| XYPoint {
                                // iterate over our data to construct a dataset
                                x: d.0.into(), // use .into() to convert to a NumberorDateString
                                y: d.1.into(),
                                ..Default::default() // always use `..Default::default()` to make sure this works in the future
                            })
                            .collect::<Vec<_>>()
                            .to_dataset_data(), // collect into a Vec<XYPoint>

                        borderColor: "blue".into(),
                        backgroundColor: "lightskyblue".into(),
                        pointRadius: 4.into(),
                        label: "Dataset 2".into(),
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
                chart.into_chart().render_mutate(); // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see bar and index.html
            })
        })
    }
    fn show_line(self: Rc<Self>, x: &[usize], y1: &[usize], y2: &[usize]) -> Dom {
        // construct and render chart here
        let id = "scatter";

        let chart = Scatter::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                datasets: Vec::from([
                    XYDataset {
                        data: x
                            .iter()
                            .zip(y1)
                            .map(|(x, y)| XYPoint {
                                // iterate over our data to construct a dataset
                                x: x.into(), // use .into() to convert to a NumberorDateString
                                y: y.into(),
                                ..Default::default() // always use `..Default::default()` to make sure this works in the future
                            })
                            .collect::<Vec<_>>()
                            .to_dataset_data(), // collect into a Vec<XYPoint>

                        borderColor: "red".into(),
                        backgroundColor: "lightcoral".into(),
                        pointRadius: 4.into(),
                        label: "Dataset 1".into(),
                        r#type: "line".into(),
                        ..Default::default() // always use `..Default::default()` to make sure this works in the future
                    },
                    XYDataset {
                        data: x
                            .iter()
                            .zip(y2)
                            .map(|(x, y)| XYPoint {
                                // iterate over our data to construct a dataset
                                x: x.into(), // use .into() to convert to a NumberorDateString
                                y: y.into(),
                                ..Default::default() // always use `..Default::default()` to make sure this works in the future
                            })
                            .collect::<Vec<_>>()
                            .to_dataset_data(), // collect into a Vec<XYPoint>

                        borderColor: "blue".into(),
                        backgroundColor: "lightskyblue".into(),
                        pointRadius: 4.into(),
                        label: "Dataset 2".into(),
                        r#type: "line".into(),
                        ..Default::default() // always use `..Default::default()` to make sure this works in the future
                    },
                ]),
                ..Default::default()
            },
            options: ChartOptions {
                scales: Some(HashMap::from([(
                    "x".into(),
                    ChartScale {
                        r#type: "linear".into(),
                        ..Default::default()
                    },
                )])),
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
                chart.into_chart().render_mutate() // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see bar and index.html
            })
        })
    }

    fn show_bar(self: Rc<Self>, data: &[usize]) -> Dom {
        // construct and render chart here
        let id = "bar";

        let chart = Bar::<NoAnnotations> {
            // we use <NoAnnotations> here to type hint for the compiler
            data: Dataset {
                labels: Some(
                    // use a range to give us our X axis labels
                    (0..data.len()).map(|d| (d + 1).into()).collect(),
                ),
                datasets: Vec::from([XYDataset {
                    data: data
                        .iter()
                        .enumerate()
                        .map(|(x, y)| XYPoint {
                            // iterate over our data to construct a dataset
                            x: (x + 1).into(), // use enumerate to give us our X axis point
                            y: y.into(),
                            ..Default::default() // always use `..Default::default()` to make sure this works in the future
                        })
                        .collect::<Vec<_>>()
                        .to_dataset_data(), // collect into a Vec<XYPoint>

                    backgroundColor: "palegreen".into(),
                    borderColor: "green".into(),
                    borderWidth: 2.into(),
                    label: "Dataset 1".into(),
                    yAxisID: "y".into(),
                    ..Default::default() // always use `..Default::default()` to make sure this works in the future
                }]),
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
                chart.to_chart().render() // use .to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see bar and index.html
            })
        })
    }

    fn show_donut(self: Rc<Self>) -> Dom {
        // construct and render chart here
        let three_id = "donut_a";
        let four_id = "donut_b";

        let three_a_chart: Doughnut<NoAnnotations> = Doughnut {
            data: {
                Dataset {
                    datasets: {
                        Vec::from([SinglePointDataset {
                            data: Vec::from([300.into(), 40.into(), 56.into(), 22.into()]),
                            backgroundColor: Vec::from([
                                "dodgerblue".into(),
                                "limegreen".into(),
                                "firebrick".into(),
                                "goldenrod".into(),
                            ]),
                            ..Default::default()
                        }])
                    },
                    labels: Some(Vec::from([
                        "Blueberries".into(),
                        "Limes".into(),
                        "Apples".into(),
                        "Lemons".into(),
                    ])),
                }
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default()
            },
            id: three_id.to_string(),
            ..Default::default()
        };
        let three_b_chart: Pie<NoAnnotations> = Pie {
            data: {
                Dataset {
                    datasets: {
                        Vec::from([SinglePointDataset {
                            data: Vec::from([300.into(), 40.into(), 56.into(), 22.into()]),
                            backgroundColor: Vec::from([
                                "dodgerblue".into(),
                                "limegreen".into(),
                                "firebrick".into(),
                                "goldenrod".into(),
                            ]),
                            ..Default::default()
                        }])
                    },
                    labels: Some(Vec::from([
                        "Blueberries".into(),
                        "Limes".into(),
                        "Apples".into(),
                        "Lemons".into(),
                    ])),
                }
            },
            options: ChartOptions {
                maintainAspectRatio: Some(false),
                ..Default::default()
            },
            id: four_id.to_string(),
            ..Default::default()
        };
        html!("div", {
            .class("columns")
            .children([
                html!("div", {
                    .class(["column", "is-half"])
                    .child(
                        html!("canvas", {
                        .prop("id", three_id)
                        .style("height", "calc(100vh - 270px)")
                        .after_inserted(move |_| {
                            three_a_chart.to_chart().render()
                        })
                    }))
                }),
                html!("div", {
                    .class(["column", "is-half"])
                    .child(
                        html!("canvas", {
                        .prop("id", four_id)
                        .style("height", "calc(100vh - 270px)")
                        .after_inserted(move |_| {
                            three_b_chart.to_chart().render()
                        })
                    }))
                })
            ])
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
                            .prop_signal("disabled", self.clone().chart_selected("donut"))
                            .text("Randomise")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    // randomise the data on button click
                                    model.clone().y1.set(Rc::new(random()));
                                    model.clone().y2.set(Rc::new(random()));
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-primary"])
                            .class_signal("is-light", self.clone().chart_not_selected("scatter"))
                            .text("Scatter")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("scatter"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-success"])
                            .class_signal("is-light", self.clone().chart_not_selected("line"))
                            .text("Line")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("line"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-primary"])
                            .class_signal("is-light", self.clone().chart_not_selected("bar"))
                            .text("Bar")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("bar"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child(
                        html!("button", {
                            .class(["button", "is-success"])
                            .class_signal("is-light", self.clone().chart_not_selected("donut"))
                            .text("Donut")
                            .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("donut"); // change which chart is in view
                                }
                            })
                        })
                    )
                    .child_signal(self.chart.signal().map(|c|
                        if c == "scatter" {
                            Some(html!("button", {
                                .class("button")
                                .prop("disabled", true)
                            }))
                        }
                        else {
                            None
                        })
                    )
                    .child_signal(self.chart.signal().map({
                        let _self = self.clone();
                        move |c|
                            if c == "scatter" {
                                Some(
                                    html!("button", {
                                        .class(["button", "is-info"])
                                        .text("Update Chart")
                                        .event({
                                            let _self = _self.clone();
                                            move |_: events::Click| {
                                                // update scatter chart colour
                                                let mut chart: Scatter::<NoAnnotations> = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
                                                chart.data.datasets.get_mut(0).map(|d| {
                                                    if _self.tick.get() {
                                                        d.backgroundColor = "lightcoral".into();
                                                        d.borderColor = "red".into();
                                                    } else {
                                                        d.backgroundColor = "palegreen".into();
                                                        d.borderColor = "green".into();
                                                    }
                                                }).unwrap();
                                                chart.into_chart().update(true);
                                                _self.tick.set(!_self.tick.get());
                                            }
                                        })
                                    })
                                )
                            }
                            else {
                                None
                            }
                        })
                    )
                    .child_signal(self.chart.signal().map({
                        let _self = self.clone();
                        move |c|
                            if c == "scatter" {
                                Some(
                                    html!("button", {
                                        .class(["button", "is-info"])
                                        .text("Update Chart without animation")
                                        .event({
                                            let _self = _self.clone();
                                            move |_: events::Click| {
                                                // update scatter chart colour
                                                let mut chart: Scatter::<NoAnnotations> = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
                                                chart.data.datasets.get_mut(0).map(|d| {
                                                    if _self.tick.get() {
                                                        d.backgroundColor = "lightcoral".into();
                                                        d.borderColor = "red".into();
                                                    } else {
                                                        d.backgroundColor = "palegreen".into();
                                                        d.borderColor = "green".into();
                                                    }
                                                }).unwrap();
                                                chart.into_chart().update(false);
                                                _self.tick.set(!_self.tick.get());
                                            }
                                        })
                                    })
                                )
                            }
                            else {
                                None
                            }
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

pub struct Mutable3<A, B, C>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
)
where
    A: Clone,
    B: Clone,
    C: Clone;
impl<A, B, C> Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>) -> Self {
        Mutable3(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
        )
    }
}
impl<A, B, C> Signal for Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    type Item = (A, B, C);

    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let mut changed = false;

        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        let middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        let right_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
            )))
        } else if left_done && middle_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}

pub struct Mutable4<A, B, C, D>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
    (MutableSignalCloned<D>, Mutable<D>),
)
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone;
impl<A, B, C, D> Mutable4<A, B, C, D>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>, d: Mutable<D>) -> Self {
        Mutable4(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
            (d.signal_cloned(), d),
        )
    }
}
impl<A, B, C, D> Signal for Mutable4<A, B, C, D>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
{
    type Item = (A, B, C, D);

    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let d = Pin::new(&mut self.3 .0).poll_change(cx);
        let mut changed = false;

        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        let left_middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_middle_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        let right_done = match d {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };

        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
                self.3 .1.get_cloned(),
            )))
        } else if left_done && left_middle_done && right_middle_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}
