use chart_js_rs::{bar::Bar, doughnut::Doughnut, pie::Pie, scatter::Scatter, *};
use dominator::{events, html, Dom};
use futures_signals::signal::{Mutable, MutableSignalCloned, Signal, SignalExt};
use itertools::Itertools;
use rand::Rng;
use std::{
    collections::BTreeMap,
    pin::Pin,
    sync::Arc,
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
    chart: Mutable<Arc<str>>,
    x: Mutable<Arc<Vec<usize>>>,
    y1: Mutable<Arc<Vec<usize>>>,
    y2: Mutable<Arc<Vec<usize>>>,
}
impl Model {
    async fn init() -> Arc<Self> {
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));

        let query_string = gloo::utils::window()
            .location()
            .search()
            .unwrap_or_default()
            .replace('?', "");
        let query = query_string
            .split('=')
            .tuples::<(&str, &str)>()
            .collect::<BTreeMap<&str, &str>>();

        Arc::new(Model {
            tick: Mutable::default(),
            chart: Mutable::new(query.get("chart").cloned().unwrap_or("scatter").into()),
            x: Mutable::new(Arc::new((0..=20).collect())),
            y1: Mutable::new(Arc::new(random())),
            y2: Mutable::new(Arc::new(random())),
        })
    }

    fn set_query(self: Arc<Model>) {
        gloo::utils::window()
            .location()
            .set_search(&format!("chart={}", self.chart.get_cloned()))
            .unwrap();
    }

    fn chart_selected(self: Arc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c.as_ref() == chart)
    }
    fn chart_not_selected(self: Arc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c.as_ref() != chart)
    }

    fn show_charts(self: Arc<Self>) -> impl Signal<Item = Option<Dom>> {
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

    fn show_scatter(self: Arc<Self>, x: &[usize], y1: &[usize], y2: &[usize]) -> Dom {
        // construct and render chart here
        let id = "scatter";

        let chart = Scatter::<NoAnnotations>::new(id)
            // we use <NoAnnotations> here to type hint for the compiler
            .data(
                Dataset::new().datasets([
                    XYDataset::new()
                        .data(x.iter().zip(y1).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        .border_color("red")
                        .background_color("lightcoral")
                        .point_radius(4)
                        .label("Dataset 1"),
                    XYDataset::new()
                        .data(x.iter().zip(y2).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        .border_color("blue")
                        .background_color("lightskyblue")
                        .point_radius(4)
                        .label("Dataset 2"),
                ]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                chart.into_chart().mutate().render();
            })
        })
    }

    fn show_line(self: Arc<Self>, x: &[usize], y1: &[usize], y2: &[usize]) -> Dom {
        // construct and render chart here
        let id = "line";

        let chart =
            Scatter::<NoAnnotations>::new(id)
                // we use <NoAnnotations> here to type hint for the compiler
                .data(Dataset::new().datasets([
                    XYDataset::new()
                        .data(
                            x.iter()
                                .zip(y1)
                                .enumerate()
                                .map(|(x, d)| {
                                    if x % 5 == 0 {
                                        ("NaN".to_string(), "NaN".to_string())
                                    } else {
                                        (d.0.to_string(), d.1.to_string())
                                    }
                                })
                                .into_data_iter()
                                .unsorted_to_dataset_data(), // collect into dataset
                        )
                        .span_gaps(true)
                        .point_radius(4)
                        .point_border_color("darkgreen")
                        .point_background_color("palegreen")
                        .label("Dataset 1")
                        .r_type("line")
                        .segment(
                            Segment::new()
                                .border_dash(FnWithArgs::new().arg("ctx").return_value(
                                    "ctx.p0.skip || ctx.p1.skip ? [2, 2] : undefined",
                                ))
                                .border_color(FnWithArgs::new().arg("ctx").return_value(
                                    "ctx.p0.skip || ctx.p1.skip ? 'lightgrey' : (ctx.p0.parsed.y > ctx.p1.parsed.y) ? 'firebrick' : 'green'"
                                )),
                        ),
                    XYDataset::new()
                        .data(x.iter().zip(y2).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        .border_color("blue")
                        .background_color("lightskyblue")
                        .point_border_color("blue")
                        .point_background_color("lightskyblue")
                        .point_radius(4)
                        .label("Dataset 2")
                        .r_type("line"),
                ]))
                .options(ChartOptions::new()
                    .scales([(
                        "x",
                        ChartScale::new()
                            .r_type("linear")
                            .ticks(ScaleTicks::new().callback(
                                FnWithArgs::new().arg("value").arg("index").return_value(
                                    "index % 2 === 0 ? this.getLabelForValue(value) : ''",
                                ),
                            )),
                    )])
                    .maintain_aspect_ratio(false)
                );
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                chart.into_chart().mutate().render();
            })
        })
    }

    fn show_bar(self: Arc<Self>, data: &[usize]) -> Dom {
        // construct and render chart here
        let id = "bar";

        let chart = Bar::<NoAnnotations>::new(id)
            // we use <NoAnnotations> here to type hint for the compiler
            .data(
                Dataset::new()
                    .labels(
                        // use a range to give us our X axis labels
                        (0..data.len()).map(|d| d + 1),
                    )
                    .datasets([XYDataset::new()
                        .data(
                            data.iter()
                                .enumerate()
                                .map(|(x, y)| ((x + 1), y))
                                .into_data_iter()
                                .unsorted_to_dataset_data(), // collect into dataset
                        )
                        .background_color("palegreen")
                        .border_color("green")
                        .border_width(2)
                        .label("Dataset 1")
                        .y_axis_id("y")]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                chart.into_chart().render() // use.to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see bar and index.html
            })
        })
    }

    fn show_donut(self: Arc<Self>) -> Dom {
        // construct and render chart here
        let three_a_id = "donut_a";
        let three_b_id = "donut_b";

        let three_a_chart = Doughnut::<NoAnnotations>::new(three_a_id)
            .data(
                Dataset::new()
                    .datasets({
                        [SinglePointDataset::new()
                            .data([300, 40, 56, 22])
                            .background_color([
                                "dodgerblue",
                                "limegreen",
                                "firebrick",
                                "goldenrod",
                            ])]
                    })
                    .labels(["Blueberries", "Limes", "Apples", "Lemons"]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        let three_b_chart = Pie::<NoAnnotations>::new(three_b_id)
            .data(
                Dataset::new()
                    .datasets({
                        [SinglePointDataset::new()
                            .data([300, 40, 56, 22])
                            .background_color([
                                "dodgerblue",
                                "limegreen",
                                "firebrick",
                                "goldenrod",
                            ])]
                    })
                    .labels(["Blueberries", "Limes", "Apples", "Lemons"]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        html!("div", {
           .class("columns")
           .children([
                html!("div", {
                   .class(["column", "is-half"])
                   .child(
                        html!("canvas", {
                       .prop("id", three_a_id)
                       .style("height", "calc(100vh - 270px)")
                       .after_inserted(move |_| {
                            three_a_chart.into_chart().render()
                        })
                    }))
                }),
                html!("div", {
                   .class(["column", "is-half"])
                   .child(
                        html!("canvas", {
                       .prop("id", three_b_id)
                       .style("height", "calc(100vh - 270px)")
                       .after_inserted(move |_| {
                            three_b_chart.into_chart().render()
                        })
                    }))
                })
            ])
        })
    }

    fn render(self: Arc<Self>) -> Dom {
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
                                    model.clone().y1.set(Arc::new(random()));
                                    model.clone().y2.set(Arc::new(random()));
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
                                    model.clone().chart.set("scatter".into()); // change which chart is in view
                                    model.clone().set_query();
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
                                    model.clone().chart.set("line".into()); // change which chart is in view
                                    model.clone().set_query();
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
                                    model.clone().chart.set("bar".into()); // change which chart is in view
                                    model.clone().set_query();
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
                                    model.clone().chart.set("donut".into()); // change which chart is in view
                                    model.clone().set_query();
                                }
                            })
                        })
                    )
                   .child_signal(self.chart.signal_cloned().map(|c|
                        if c.as_ref() == "scatter" {
                            Some(html!("button", {
                               .class("button")
                               .prop("disabled", true)
                            }))
                        }
                        else {
                            None
                        })
                    )
                   .child_signal(self.chart.signal_cloned().map({
                        let _self = self.clone();
                        move |c|
                            if c.as_ref() == "scatter" {
                                Some(
                                    html!("button", {
                                       .class(["button", "is-info"])
                                       .text("Update Chart")
                                       .event({
                                            let _self = _self.clone();
                                            move |_: events::Click| {
                                                // update scatter chart colour
                                                let mut chart: Scatter::<NoAnnotations> = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
                                                chart.get_data().get_datasets().get_mut(0).map(|d| {
                                                    if _self.tick.get() {
                                                        *d.get_background_color() = "lightcoral".into();
                                                        *d.get_border_color() = "red".into();
                                                    } else {
                                                        *d.get_background_color() = "palegreen".into();
                                                        *d.get_border_color() = "green".into();
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
                   .child_signal(self.chart.signal_cloned().map({
                        let _self = self.clone();
                        move |c|
                            if c.as_ref() == "scatter" {
                                Some(
                                    html!("button", {
                                       .class(["button", "is-info"])
                                       .text("Update Chart without animation")
                                       .event({
                                            let _self = _self.clone();
                                            move |_: events::Click| {
                                                // update scatter chart colour
                                                let mut chart: Scatter::<NoAnnotations> = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
                                                chart.get_data().get_datasets().get_mut(0).map(|d| {
                                                    if _self.tick.get() {
                                                        *d.get_background_color() = "lightcoral".into();
                                                        *d.get_border_color() = "red".into();
                                                    } else {
                                                        *d.get_background_color() = "palegreen".into();
                                                        *d.get_border_color() = "green".into();
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
