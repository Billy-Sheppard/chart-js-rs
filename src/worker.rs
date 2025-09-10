use gloo_console::console_dbg;
use gloo_utils::{document, window};
use itertools::Itertools;
use js_sys::{Array, Function, Object, Reflect};
use tokio::sync::broadcast::{self};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{MessageEvent, Worker, WorkerOptions, WorkerType};

type Callback = dyn FnMut(MessageEvent);

const WORKER_SHIM: &str = include_str!("../js/worker_shim.js");

macro_rules! obj {
    ($($key:literal => $val:expr),+ $(,)?) => {
        {
            let object = Object::new();
            $(
                let _ = Reflect::set(&object, &$key.into(), &{$val}.into());
            )*
            object
        }
    };
}

macro_rules! destructure {
    ($obj:expr => $($key:ident),+ $(,)?) => {
        $(
            let $key = Reflect::get(&$obj, &stringify!($key).into());
        )*
    };
}

macro_rules! if_let_all {
    ($($var:ident),+ => $body:block) => {
    if let Some(($($var,)+)) = [$($var),+]
        .into_iter()
        .collect::<Result<Vec<_>, _>>()
        .ok()
        .and_then(|v| v.into_iter().collect_tuple()) {
        $body
    }
    };
}

#[derive(Clone, Debug)]
pub(crate) enum MessageContent {
    CallbackRequest {
        function: String,
        args: Array,
    },

    CallbackResponse {
        data: JsValue,
    },

    Render {
        canvas: JsValue,
        width: JsValue,
        height: JsValue,
        obj: JsValue,
        id: String,
        mutate: bool,
        plugins: String,
        defaults: String,
    },

    Update {
        updated: JsValue,
        id: String,
        animate: bool,
    },

    Other,
}

impl From<MessageContent> for JsValue {
    fn from(val: MessageContent) -> Self {
        match val {
            MessageContent::CallbackRequest { function, args } => obj! {
                "function" => function,
                "args" => args
            },
            MessageContent::CallbackResponse { data } => obj! {
                "data" => data
            },
            MessageContent::Render {
                canvas,
                width,
                height,
                obj,
                id,
                mutate,
                plugins,
                defaults,
            } => obj! {
                "canvas" => canvas,
                "width" => width,
                "height" => height,
                "obj" => obj,
                "id" => id,
                "mutate" => mutate,
                "plugins" => plugins,
                "defaults" => defaults,
            },
            MessageContent::Update {
                updated,
                id,
                animate,
            } => obj! {
                "updated" => updated,
                "id" => id,
                "animate" => animate,
            },

            _ => return Array::new().into(),
        }
        .into()
    }
}

impl From<JsValue> for MessageContent {
    fn from(value: JsValue) -> Self {
        // console_dbg!(value);

        if !value.is_object() {
            return Self::Other;
        }

        destructure!(
            value => function, args, data, canvas, width, height, obj, id, mutate, plugins, defaults, updated, animate
        );

        if_let_all!(function, args => {
            return MessageContent::CallbackRequest {
                function: function.as_string().unwrap_or_default(),
                args: Array::from(&args),
            };
        });

        if let Ok(data) = data {
            return MessageContent::CallbackResponse { data };
        }

        {
            let id = id.clone();
            if_let_all!(canvas, width, height, obj, id, mutate, plugins, defaults => {
                return MessageContent::Render {
                    canvas,
                    width,
                    height,
                    obj,
                    id: id.as_string().unwrap_or_default(),
                    mutate: mutate.as_bool().unwrap_or_default(),
                    plugins: plugins.as_string().unwrap_or_default(),
                    defaults: defaults.as_string().unwrap_or_default(),
                };
            });
        }

        if_let_all!(updated, id, animate => {
            return MessageContent::Update {
                updated,
                id: id.as_string().unwrap_or_default(),
                animate: animate.as_bool().unwrap_or_default(),
            };
        });

        MessageContent::Other
    }
}

#[derive(Clone)]
pub struct ChartWorker {
    pub(crate) worker: Worker,
    pub(crate) from_worker: broadcast::Sender<(String, MessageContent)>,
}

impl ChartWorker {
    pub(crate) async fn render(
        &self,
        obj: JsValue,
        chart_id: &String,
        mutate: bool,
        plugins: String,
        defaults: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let canvas_element = document().get_element_by_id(chart_id).unwrap();

        let width = canvas_element.client_width();
        let height = canvas_element.client_height();
        let rect = canvas_element.get_bounding_client_rect();
        let computed_styles = web_sys::window()
            .unwrap()
            .get_computed_style(&canvas_element)
            .unwrap()
            .unwrap();

        let mousemove_handler = {
            let worker = self.clone();
            let chart_id = chart_id.clone();
            Closure::wrap(Box::new({
                let computed_styles = computed_styles.clone();
                let rect = rect.clone();
                move |event: web_sys::MouseEvent| {
                    let x = (event.client_x() as f64 - rect.left()) * (width as f64 / rect.width());
                    let y =
                        (event.client_y() as f64 - rect.top()) * (height as f64 / rect.height());

                    let message = js_sys::Object::new();
                    js_sys::Reflect::set(&message, &"type".into(), &"mouse-event".into()).unwrap();
                    js_sys::Reflect::set(&message, &"eventType".into(), &"mousemove".into())
                        .unwrap();
                    js_sys::Reflect::set(&message, &"x".into(), &x.into()).unwrap();
                    js_sys::Reflect::set(&message, &"y".into(), &y.into()).unwrap();
                    js_sys::Reflect::set(&message, &"chartId".into(), &chart_id.clone().into())
                        .unwrap();

                    // Add computed styles
                    let styles_obj = js_sys::Object::new();
                    js_sys::Reflect::set(
                        &styles_obj,
                        &"fontFamily".into(),
                        &computed_styles
                            .get_property_value("font-family")
                            .unwrap()
                            .into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(
                        &styles_obj,
                        &"fontSize".into(),
                        &computed_styles
                            .get_property_value("font-size")
                            .unwrap()
                            .into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(
                        &styles_obj,
                        &"fontWeight".into(),
                        &computed_styles
                            .get_property_value("font-weight")
                            .unwrap()
                            .into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(
                        &styles_obj,
                        &"fontStyle".into(),
                        &computed_styles
                            .get_property_value("font-style")
                            .unwrap()
                            .into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(
                        &styles_obj,
                        &"lineHeight".into(),
                        &computed_styles
                            .get_property_value("line-height")
                            .unwrap()
                            .into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(
                        &styles_obj,
                        &"color".into(),
                        &computed_styles.get_property_value("color").unwrap().into(),
                    )
                    .unwrap();
                    js_sys::Reflect::set(&message, &"computedStyles".into(), &styles_obj).unwrap();

                    worker.worker.post_message(&message).unwrap();
                }
            }) as Box<dyn FnMut(_)>)
        };

        let mouseleave_handler = {
            let worker = self.clone();
            let chart_id = chart_id.clone();
            Closure::wrap(Box::new(move |_event: web_sys::MouseEvent| {
                let message = js_sys::Object::new();
                js_sys::Reflect::set(&message, &"type".into(), &"mouse-event".into()).unwrap();
                js_sys::Reflect::set(&message, &"eventType".into(), &"mouseleave".into()).unwrap();
                js_sys::Reflect::set(&message, &"chartId".into(), &chart_id.clone().into())
                    .unwrap();

                worker.worker.post_message(&message).unwrap();
            }) as Box<dyn FnMut(_)>)
        };
        let click_handler = {
            let worker = self.clone();
            let chart_id = chart_id.clone();
            let computed_styles = computed_styles.clone();
            let rect = rect.clone();
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                let x = (event.client_x() as f64 - rect.left()) * (width as f64 / rect.width());
                let y = (event.client_y() as f64 - rect.top()) * (height as f64 / rect.height());

                let message = js_sys::Object::new();
                js_sys::Reflect::set(&message, &"type".into(), &"mouse-event".into()).unwrap();
                js_sys::Reflect::set(&message, &"eventType".into(), &"click".into()).unwrap();
                js_sys::Reflect::set(&message, &"x".into(), &x.into()).unwrap();
                js_sys::Reflect::set(&message, &"y".into(), &y.into()).unwrap();
                js_sys::Reflect::set(&message, &"chartId".into(), &chart_id.clone().into())
                    .unwrap();

                // Add computed styles (same as mousemove)
                let styles_obj = js_sys::Object::new();
                js_sys::Reflect::set(
                    &styles_obj,
                    &"fontFamily".into(),
                    &computed_styles
                        .get_property_value("font-family")
                        .unwrap()
                        .into(),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &styles_obj,
                    &"fontSize".into(),
                    &computed_styles
                        .get_property_value("font-size")
                        .unwrap()
                        .into(),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &styles_obj,
                    &"fontWeight".into(),
                    &computed_styles
                        .get_property_value("font-weight")
                        .unwrap()
                        .into(),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &styles_obj,
                    &"fontStyle".into(),
                    &computed_styles
                        .get_property_value("font-style")
                        .unwrap()
                        .into(),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &styles_obj,
                    &"lineHeight".into(),
                    &computed_styles
                        .get_property_value("line-height")
                        .unwrap()
                        .into(),
                )
                .unwrap();
                js_sys::Reflect::set(
                    &styles_obj,
                    &"color".into(),
                    &computed_styles.get_property_value("color").unwrap().into(),
                )
                .unwrap();
                js_sys::Reflect::set(&message, &"computedStyles".into(), &styles_obj).unwrap();

                worker.worker.post_message(&message).unwrap();
            }) as Box<dyn FnMut(_)>)
        };

        canvas_element
            .add_event_listener_with_callback(
                "mousemove",
                mousemove_handler.as_ref().unchecked_ref(),
            )
            .unwrap();
        canvas_element
            .add_event_listener_with_callback(
                "mouseleave",
                mouseleave_handler.as_ref().unchecked_ref(),
            )
            .unwrap();
        canvas_element
            .add_event_listener_with_callback("click", click_handler.as_ref().unchecked_ref())
            .unwrap();

        mousemove_handler.forget();
        mouseleave_handler.forget();
        click_handler.forget();

        let offscreen_canvas = canvas_element
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap()
            .transfer_control_to_offscreen()
            .unwrap();

        self.send(
            MessageContent::Render {
                canvas: offscreen_canvas.clone().into(),
                width: width.into(),
                height: height.into(),
                obj,
                id: chart_id.to_string(),
                mutate,
                plugins,
                defaults,
            },
            &[offscreen_canvas.into()],
        )
        .await
        .map(|_| ())
    }

    pub(crate) async fn update(
        &self,
        updated: JsValue,
        id: &String,
        animate: bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        self.send(
            MessageContent::Update {
                updated,
                id: id.to_string(),
                animate,
            },
            &[],
        )
        .await
        .map(|v| v.as_bool().unwrap_or_default())
    }

    pub(crate) async fn new(imports_block: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // Spawn Worker
        let worker_options = WorkerOptions::new();
        worker_options.set_type(WorkerType::Module);

        let from_worker = broadcast::channel::<(String, MessageContent)>(1).0;
        let worker = Worker::new_with_options(&shim_blob(imports_block), &worker_options)
            .map_err(|e| format!("{e:?}"))?;

        let handler = {
            let from_worker = from_worker.clone();
            let worker = worker.clone();
            Closure::<Callback>::new(move |ev: MessageEvent| {
                let data = Array::from(&ev.data());
                let id = data.get(0).as_string().unwrap_or_default();
                let message: MessageContent = data.get(1).into();

                match message.clone() {
                    MessageContent::CallbackRequest { function, args } => {
                        console_dbg!(message);

                        let callbacks =
                            Reflect::get(&window().into(), &"callbacks".into()).unwrap();
                        let function = Reflect::get(&callbacks, &function.into()).unwrap();
                        let function = Function::from(function);

                        let data = function.apply(&JsValue::null(), &args).unwrap();
                        let _ = worker.post_message(&{
                            let arr = Array::new();
                            arr.push(&id.into());
                            arr.push(&MessageContent::CallbackResponse { data }.into());
                            arr.into()
                        });
                    }
                    _ => {
                        let _ = from_worker.send((id, message));
                    }
                }
            })
        };

        worker.set_onmessage(Some(handler.as_ref().unchecked_ref()));
        handler.forget();
        from_worker.subscribe().recv().await?;

        // Return tx
        Ok(Self {
            worker,
            from_worker,
        })
    }

    async fn send(
        &self,
        v: MessageContent,
        t: &[JsValue],
    ) -> Result<JsValue, Box<dyn std::error::Error>> {
        let ts = uuid::Uuid::new_v4().to_string();
        let arr = Array::new();
        arr.push(&ts.clone().into());
        arr.push(&v.into());

        self.worker
            .post_message_with_transfer(&arr.into(), &Array::from_iter(t.iter()))
            .map_err(|e| format!("{e:?}"))?;

        loop {
            if let Ok((id, data)) = self.from_worker.subscribe().recv().await {
                let arr = Array::from(&data.into());
                let data = arr.get(1);

                if id == ts {
                    return Ok(data);
                }
            }
        }
    }
}

fn shim_blob(imports_block: &str) -> String {
    let _imports = js_sys::eval(
        "[...document.head.querySelectorAll('script')].map(s => s.src).filter(Boolean)",
    )
    .map(|v| Array::from(&v))
    .unwrap_or_default();

    let shim = WORKER_SHIM.replace("/// IMPORTS", imports_block);

    web_sys::Url::create_object_url_with_blob(
        &web_sys::Blob::new_with_blob_sequence_and_options(
            &{
                let a = Array::new();
                a.push(&shim.into());
                a.into()
            },
            &{
                let bag = web_sys::BlobPropertyBag::new();
                bag.set_type("application/javascript");
                bag
            },
        )
        .unwrap(),
    )
    .unwrap()
}
