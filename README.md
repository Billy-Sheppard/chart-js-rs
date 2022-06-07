# Chart.js types API in Rust

***In Alpha, types added as needed, feel free to PR.***

## How to use

### Cargo.toml: 
```toml
[dependencies.chart-js-rs]
path = "../../../chart-js-rs"
```

### Rust:
```rust
    let id = "[YOUR CHART ID HERE]";
    let chart = chart_js_rs::scatter::Scatter {
        id: id.to_string(),
        r#type: "scatter".into(),
        options: ChartOptions { .. },
        data: Dataset { .. },
    };
    // to use any callbacks or functions you use render_mutate and refer to the JS below
    chart.to_chart().render_mutate(&id);

    // else use render
    chart.to_chart().render(id);
```

### Your html file:
```html
<script src="https://cdn.jsdelivr.net/npm/chart.js@^3"></script>

...

<script>
  function mutate_chart_object(v) { // must have this function name
    if (v.id === ("[YOUR CHART ID HERE]")) {
    // do any work here, this would prepend `$` to y1 axis tick labels
      v.options.scales.y1.ticks = {
        callback:
          function (value, _index, _values) {
            return '$' + value.toFixed(2);
          }
      };
    };

    return v
  }
</script>
```