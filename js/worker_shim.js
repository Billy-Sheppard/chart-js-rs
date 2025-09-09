console.log('Chart worker ready');

self.window = {
    callbacks: {} //callbacks
};

(async () => {
    /// IMPORTS
})().then(() => {
    const is_obj = x => typeof x === 'object' && !Array.isArray(x) && x !== null;
    const derationalize = (o) => {

        // Handle arrays separately
        if (Array.isArray(o)) {
            return o.map((item, index) => {
                return derationalize(item);
            });
        }

        if (!is_obj(o)) {
            return o;
        } else if ('args' in o && 'body' in o && 'closure_id' in o && 'return_value' in o) {
            let { args, body, closure_id, return_value } = o;

            if (closure_id) {
                return Function(...args, `{ 
                return 'orange';
            }`);
            } else {
                return Function(...args, `{ ${body} \n return ${return_value} }`);
            }
        } else {
            return Object.fromEntries(Object.entries(o).map(v => {
                return [v[0], derationalize(v[1])]
            }))
        }
    }

    // console.log("sending ready");
    self.postMessage("");

    self.onmessage = (async event => {
        try {
            let [transaction, data] = event.data ?? [];
            // console.log(transaction, data);

            let {
                canvas, width, height,
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
                console.log('New Chart');

                canvas.width = width;
                canvas.height = height;

                const chart = new Chart(canvas, obj);
                chart.resize();
            } else {
                try {
                    let chart = Chart.getChart(canvas);
                    chart.config._config.type = updated.type;
                    chart.config._config.data = updated.data;
                    chart.config._config.options = updated.options;

                    console.debug('Updated chart:', chart);

                    if (animate) {
                        chart.update();
                        chart.resize();
                    } else {
                        chart.update('none');
                    }
                } catch {
                    // console.log("sending update failure");
                    postMessage([transaction, false])
                }
            }

            // console.log("sending success");
            postMessage([transaction, true])
        } catch (e) {
            console.error(e);
        }
    })
})