console.log('Chart worker ready');

(async () => {
    /// IMPORTS
    // Import Chart.js first
    await import("https://cdn.jsdelivr.net/npm/chart.js@4.4.0/dist/chart.umd.js");
    
    // Import Luxon with explicit global setup
    await import("https://cdn.jsdelivr.net/npm/luxon@^2/build/global/luxon.min.js");
    
    // Ensure luxon is properly set up globally
    if (typeof self.luxon === 'undefined') {
        // If the global version didn't work, try importing the ESM version
        const luxonESM = await import("https://cdn.jsdelivr.net/npm/luxon@^2/+esm");
        self.luxon = luxonESM;
    }
    
    // Verify DateTime and its constants are available
    if (self.luxon && self.luxon.DateTime && self.luxon.DateTime.DATETIME_MED_WITH_SECONDS) {
        console.log('DateTime constants available');
    } else {
        console.error('DateTime constants not available');
    }
    
    // Now import the adapter
    await import("https://cdn.jsdelivr.net/npm/chartjs-adapter-luxon@^1/dist/chartjs-adapter-luxon.umd.min.js");
})().then(() => {
    const window = {
        mutate_chart_object: function (v) {
            if (v.id == "bar") {
                v.options.scales.y1.ticks = {
                    callback:
                        function (value, _index, _values) {
                            return '$' + value.toFixed(2);
                        }
                };
            }
            return v
        },

        callbacks: {} //callbacks
    }

    const is_obj = x => typeof x === 'object' && !Array.isArray(x) && x !== null;
    const derationalize = (o) => {
        if (!is_obj(o)) {
            return o;
        } else if ('args' in o && 'body' in o && 'closure_id' in o && 'return_value' in o) {
            let { args, body, closure_id, return_value } = o;
            console.debug();

            if (closure_id) {
                return Function(...args, `{ return window.callbacks['${closure_id}'](${args.join(', ')}) }`);

            } else {
                return Function(...args, `{ ${body} \n return ${return_value} }`);
            }
        } else {
            return Object.fromEntries(Object.entries(o).map(v => {
                return [v[0], derationalize(v[1])]
            }))
        }
    }

    console.log("sending ready");
    self.postMessage("");

    self.onmessage = (async event => {
        try {
            let [transaction, data] = event.data ?? [];
            console.log(transaction, data);

            let {
                canvas,
                obj, mutate, plugins, defaults, // render()
                updated, animate                // update()
            } = (data ?? {});

            if (obj) {
                if (defaults != null || defaults != undefined) {
                    defaults = eval(defaults);
                }

                if (plugins != null || plugins != undefined) {
                    obj.plugins = eval(plugins);
                }

                if (mutate) {
                    obj = window.mutate_chart_object(obj)
                }
            }

            if (obj) obj = derationalize(obj);
            if (updated) updated = derationalize(updated);

            if (!updated) {
                console.log('new chart', canvas, obj)
                const chart = new Chart(canvas, obj);
                console.log('chart newed')
                // const { width, height } = canvas.getClientBoundingRect();

                // canvas.width = width;
                // canvas.height = height;
                // chart.resize();
            } else {
                try {
                    let chart = Chart.getChart(canvas);
                    chart.config._config.type = updated.type;
                    chart.config._config.data = updated.data;
                    chart.config._config.options = updated.options;

                    console.debug('Updated chart:', chart);

                    if (animate) {
                        chart.update();
                    } else {
                        chart.update('none');
                    }
                } catch {
                    console.log("sending update failure");
                    postMessage([transaction, false])
                }
            }

            console.log("sending success");
            postMessage([transaction, true])
        } catch (e) {
            console.error(e);
        }
    })
})