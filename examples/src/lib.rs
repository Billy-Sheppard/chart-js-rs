use chart_js_rs::{bar::Bar, doughnut::Doughnut, pie::Pie, scatter::Scatter, *};
use dominator::{events, html, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use itertools::Itertools;
use rand::{Rng, SeedableRng};
use std::{collections::BTreeMap, sync::Arc};
use utils::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use wasm_bindgen_futures::spawn_local;

mod utils;

fn random() -> Vec<usize> {
    let rnd = (0..=20).map(|_| {
        let mut buf: [u8; 32] = Default::default();
        getrandom::getrandom(&mut buf).unwrap();
        let mut rng = rand::prelude::StdRng::from_seed(buf);

        rng.random_range(1..50)
    });

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
            "line-async" => Some(self.clone().show_line_async(
                x.iter().as_slice(),
                y1.as_slice(),
                y2.as_slice(),
            )),
            _ => None,
        })
    }

    fn show_scatter(self: Arc<Self>, x: &[usize], y1: &[usize], y2: &[usize]) -> Dom {
        // construct and render chart here
        let id = "scatter";

        let chart = Scatter::new(id)
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

        let chart = Scatter::new(id)
            // we use <NoAnnotations> here to type hint for the compiler
            .data(
                Dataset::new().datasets([
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
                        .dataset_type("line")
                        .segment(
                            Segment::new()
                                .border_dash(
                                    // one way is to write your logic in Javascript
                                    FnWithArgs::new()
                                        .args(["ctx"])
                                        .js_body(
                                            "if (ctx.p0.skip || ctx.p1.skip) {
                                                var out = [2, 2]
                                            } else {
                                                var out = undefined
                                            };",
                                        )
                                        .js_return_value("out"),
                                )
                                .border_color(
                                    // alternatively you can pass a closure with the same amount of arguments as the FnWithArgs<N>
                                    FnWithArgs::new().args(["ctx"]).rust_closure(|ctx| {
                                        let ctx = uncircle_chartjs_value_to_serde_json_value(ctx)
                                            .unwrap();

                                        if ctx["p0"]["skip"].as_bool().unwrap()
                                            || ctx["p1"]["skip"].as_bool().unwrap()
                                        {
                                            "lightgrey"
                                        } else if ctx["p0"]["parsed"]["y"].as_i64()
                                            > ctx["p1"]["parsed"]["y"].as_i64()
                                        {
                                            "firebrick"
                                        } else {
                                            "green"
                                        }
                                        .into()
                                    }),
                                ),
                        ),
                    XYDataset::new()
                        .data(x.iter().zip(y2).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        .border_color("blue")
                        .background_color("lightskyblue")
                        .point_border_color("blue")
                        .point_background_color("lightskyblue")
                        .point_radius(4)
                        .label("Dataset 2")
                        .dataset_type("line"),
                ]),
            )
            .options(
                ChartOptions::new()
                    .scales([(
                        "x",
                        ChartScale::new().scale_type("linear").ticks(
                            ScaleTicks::new().callback(
                                // we can call rust functions in callbacks
                                FnWithArgs::<3>::new()
                                    // we can override any arguments going in, in this case we must as rust cannot handle `this`.
                                    // Note: if you don't define your variables with ``.args([..])`, they get the default label of the letter of the alphabet they're the index of
                                    //       1st arg: `a`
                                    //       2nd arg: `b`
                                    //       ...
                                    .js_body("var a = this.getLabelForValue(a);")
                                    // function pointer goes here - note that the count of arguments must equal the const param (3 in this case)
                                    .run_rust_fn(show_line_ticks),
                            ),
                        ),
                    )])
                    .maintain_aspect_ratio(false),
            );
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                chart.into_chart().mutate().render();
            })
        })
    }

    fn show_line_async(self: Arc<Self>, x: &[usize], y1: &[usize], y2: &[usize]) -> Dom {
        // construct and render chart here
        let id = "line-async";

        let chart = Scatter::new(id)
            // we use <NoAnnotations> here to type hint for the compiler
            .data(
                Dataset::new().datasets([
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
                        .dataset_type("line")
                        .segment(
                            Segment::new()
                                .border_dash(
                                    // one way is to write your logic in Javascript
                                    FnWithArgs::new()
                                        .args(["ctx"])
                                        .js_body(
                                            "if (ctx.p0.skip || ctx.p1.skip) {
                                                var out = [2, 2]
                                            } else {
                                                var out = undefined
                                            };",
                                        )
                                        .js_return_value("out"),
                                )
                                .border_color(
                                    // alternatively you can pass a closure with the same amount of arguments as the FnWithArgs<N>
                                    FnWithArgs::new().args(["ctx"]).rust_closure(|ctx| {
                                        let ctx = uncircle_chartjs_value_to_serde_json_value(ctx)
                                            .unwrap();

                                        if ctx["p0"]["skip"].as_bool().unwrap()
                                            || ctx["p1"]["skip"].as_bool().unwrap()
                                        {
                                            "lightgrey"
                                        } else if ctx["p0"]["parsed"]["y"].as_i64()
                                            > ctx["p1"]["parsed"]["y"].as_i64()
                                        {
                                            "firebrick"
                                        } else {
                                            "green"
                                        }
                                        .into()
                                    }),
                                ),
                        ),
                    XYDataset::new()
                        .data(x.iter().zip(y2).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        .border_color("blue")
                        .background_color("lightskyblue")
                        .point_border_color("blue")
                        .point_background_color("lightskyblue")
                        .point_radius(4)
                        .label("Dataset 2")
                        .dataset_type("line"),
                ]),
            )
            .options(
                ChartOptions::new()
                    .scales([(
                        "x",
                        ChartScale::new().scale_type("linear").ticks(
                            ScaleTicks::new().callback(
                                // we can call rust functions in callbacks
                                FnWithArgs::<3>::new()
                                    // we can override any arguments going in, in this case we must as rust cannot handle `this`.
                                    // Note: if you don't define your variables with ``.args([..])`, they get the default label of the letter of the alphabet they're the index of
                                    //       1st arg: `a`
                                    //       2nd arg: `b`
                                    //       ...
                                    .js_body("var a = this.getLabelForValue(a);")
                                    // function pointer goes here - note that the count of arguments must equal the const param (3 in this case)
                                    .run_rust_fn(show_line_ticks),
                            ),
                        ),
                    )])
                    .maintain_aspect_ratio(false),
            );
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                spawn_local(async {
                    gloo::console::log!("Starting render...");
                    chart.into_worker_chart().await.unwrap().mutate().render_async().await.unwrap();
                    gloo::console::log!("Completed render!");
                });
            })
        })
    }

    fn show_bar(self: Arc<Self>, data: &[usize]) -> Dom {
        // construct and render chart here
        let id = "bar";

        let chart = Bar::<Vec<_>>::new(id)
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

        let three_a_chart = Doughnut::new(three_a_id)
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
        let three_b_chart = Pie::new(three_b_id)
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
                           .class(["button", "is-success"])
                           .class_signal("is-light", self.clone().chart_not_selected("line-async"))
                           .text("Line (Async)")
                           .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("line-async".into()); // change which chart is in view
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
                                                let mut chart: Scatter = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
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
                                                let mut chart: Scatter = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
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

#[wasm_bindgen]
pub fn show_line_ticks(this: String, index: u32, _ticks: JsValue) -> String {
    if index % 2 == 0 {
        this
    } else {
        String::new()
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let app = Model::init().await;
    dominator::append_dom(&dominator::body(), Model::render(app));
    Ok(())
}
