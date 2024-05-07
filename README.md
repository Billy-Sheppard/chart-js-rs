<p align="center">
  <img width="200" src="https://raw.githubusercontent.com/Billy-Sheppard/chart-js-rs/main/examples/favicon.png" alt="Material Bread logo">
</p>

# Chart.js types API in Rust 
[![crates.io](https://img.shields.io/crates/v/chart-js-rs.svg)](https://crates.io/crates/chart-js-rs)
[![docs.rs](https://docs.rs/chart-js-rs/badge.svg)](https://docs.rs/chart-js-rs)

***In Alpha, types added as needed, feel free to PR.***

## How to use

Check out the example folder for some code examples. The example uses WebAssembly and the [dominator](https://github.com/Pauan/rust-dominator) crate to produce charts. This library should be compatible with any WASM/HTML library.

The compiled webpage can be found here: https://billy-sheppard.github.io/chart-js-rs/examples/index.html

### Cargo.toml: 
```toml
[dependencies.chart-js-rs]
git = "https://github.com/Billy-Sheppard/chart-js-rs"
```

### Rust:
```rust
    let id = "[YOUR CHART ID HERE]";
    let chart = chart_js_rs::scatter::Scatter {
        id: id.to_string(),
        options: ChartOptions { .. },
        data: Dataset { .. },
        ..Default::default()
    };
    // to use any JS callbacks or functions you use render_mutate and refer to the JS below
    chart.to_chart().mutate().render();

    // to use any chart-js plugins, a few examples
    chart.to_chart().plugins("[window['chartjs-plugin-autocolors']]").render(); // for autocolors and no mutating
    chart.to_chart().mutate().plugins("[window['chartjs-plugin-autocolors']]").render(); // for autocolors and mutating
    chart.to_chart().mutate().plugins("[ChartDataLabels, window['chartjs-plugin-autocolors']]").render(); // for datalabels, autocolors, and mutating

    // else use render
    chart.to_chart().render();
```

### Your html file:
```html
<script src="https://cdn.jsdelivr.net/npm/chart.js@^4"></script>

...

<script type="module">
    import init from 'wasm.js';

    async function run() {
      await init();
    }

    run();
</script>

...

<script type="module">
  import * as root from './chart_js_rs_example.js'

  window.callbacks = root;
  window.mutate_chart_object = function (v) {
    if (v.id === ("[YOUR CHART ID HERE]")) {
      v.options.scales.y1.ticks = {
        callback:
          function (value, _index, _values) {
            return '$' + value.toFixed(2);
          }
      };
    }

    return v
  };
</script>
```

<hr>

# Explainers

## Whats the difference between `my_chart.render()` and `mychart.mutate().render()`?
`.render()` is for simple charts, that don't require any further changes done using javascript code.

`.mutate().render()` allows for chart objects to be accessible in your javascript file, so you can mutate the object however required, especially useful for ChartJS functions not yet available in this library.

`.plugins("[MyPlugin]").mutate().render()` allows for ChartJS plugins to be used with your charts, the parameter is the direct string representation of the Javascript array containing your plugins. [Docs](https://www.chartjs.org/docs/latest/developers/plugins.html)

## How to use `struct FnWithArgs`?
`FnWithArgs` is a helper struct to allow serialization of javascript functions by encoding their body and arguments as a string. Then, as needed, the function can be rebuilt in JavaScipt, and called.

It is important then, that you know which variables are being parsed to the function. For this information, you can refer to the [Chart.js documentation](https://www.chartjs.org/docs/latest/).

`FnWithArgs` is used, for example, in implementing conditional line segment colouring, according to the [docs](https://www.chartjs.org/docs/latest/samples/line/segments.html). 
It can also be used to leaverage logic written in Rust, to calculate outputs for ChartJS.
```rust
  #[wasm_bindgen] // your function must be a wasm_bindgen export
  pub fn add(a: u32, b: u32) -> u32 {
      a + b
  }

  // ...

  Scatter::</*...*/> {
    data: {
      datasets: vec![
        Dataset {
          // ...
          segment: Segment {
            borderColor: 
              FnWithArgs::new() // Create the Function
                .arg("ctx") // Add a named argument using a builder pattern, you can have as many arugments as required

                // run rust fn takes your input params, output variable name, and function pointer
                // this will produce 
                // const output = window.callbacks.add(1, 1);
                // return ctx.p0.parsed.y > ctx.p1.parsed.y ? 'red' : 'green'
                .run_rust_fn(&["1".into(), "1".into()], "output".into(), add) 

                // .body() can be used to add any other javascript
                .body("console.log(output)")

                .return_value("ctx.p0.parsed.y > ctx.p1.parsed.y ? 'red' : 'green'") // Add the function body, in this case make the line red if the slope is negative

                // this will produce
                // function(ctx) {
                //   const output = windows.callbacks.add(1, 1);
                //   console.log(output);
                //   return ctx.p0.parsed.y > ctx.p1.parsed.y ? 'red' : 'green'
                // }
          }
        }
      ]
    }
  }
```
