const chartAreaBackground = {
    id: 'canvas_background_color',
    beforeDraw: (chart, args, options) => {
        const { ctx, width, height } = chart;
        ctx.save();
        ctx.fillStyle = options.color || 'white'; // default to white
        ctx.fillRect(0, 0, width, height); // fill entire canvas
        ctx.restore();
    }
};

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

    // Store chart instances for mouse event handling
    let chartInstances = new Map();

    // Store computed styles for each chart
    let chartComputedStyles = new Map();

    // Handle mouse events from main thread
    function handleChartMouseEvent(eventType, x, y, chartId, computedStyles) {
        const chart = chartInstances.get(chartId);
        if (!chart) {
            console.warn(`Chart with id ${chartId} not found for mouse event`);
            return;
        }

        // Store the real computed styles from main thread
        if (computedStyles) {
            chartComputedStyles.set(chartId, computedStyles);
        }

        // Mock DOM properties that Chart.js expects in worker context
        if (!chart.canvas.ownerDocument) {
            chart.canvas.ownerDocument = {
                defaultView: {
                    getComputedStyle: function (element, pseudoElement) {
                        const styles = chartComputedStyles.get(chartId);

                        if (styles) {
                            styles.getPropertyValue = function (prop) {
                                // Convert kebab-case to camelCase
                                const camelProp = prop.replace(/-([a-z])/g, (match, letter) => letter.toUpperCase());
                                return styles[camelProp] || styles[prop];
                            };
                        }

                        return styles;
                    }
                }
            };
        }

        // Create synthetic event that Chart.js expects
        const syntheticEvent = {
            type: eventType,
            x: x,
            y: y,
            offsetX: x,
            offsetY: y,
            target: chart.canvas,
            currentTarget: chart.canvas,
            preventDefault: function () { },
            stopPropagation: function () { }
        };

        if (eventType === 'mousemove') {
            // Use Chart.js's internal event handling to match main thread behavior
            const elements = chart.getElementsAtEventForMode(
                syntheticEvent,
                chart.options.interaction?.mode || 'nearest',
                chart.options.interaction || { intersect: false },
                false
            );

            // Let Chart.js handle the tooltip logic exactly as it would normally
            chart.tooltip.setActiveElements(elements, syntheticEvent);
            chart.draw();

        } else if (eventType === 'mouseleave') {
            // Clear tooltip when mouse leaves - use same position format as mousemove
            chart.tooltip.setActiveElements([], syntheticEvent);
            chart.draw();

        } else if (eventType === 'click') {
            // Handle click events - this includes legend clicks and data point clicks
            // console.log('Click event received in worker:', x, y, chartId);
            // console.log('Chart instance found:', !!chart);

            // For legend clicks, we need to check if the click is within the legend area
            const legend = chart.legend;
            if (legend && legend.legendHitBoxes) {
                // Check if click is within any legend item
                for (let i = 0; i < legend.legendHitBoxes.length; i++) {
                    const hitBox = legend.legendHitBoxes[i];
                    if (x >= hitBox.left && x <= hitBox.left + hitBox.width &&
                        y >= hitBox.top && y <= hitBox.top + hitBox.height) {
                        // console.log('Legend item clicked:', i);
                        // Manually toggle the dataset
                        const meta = chart.getDatasetMeta(hitBox.datasetIndex !== undefined ? hitBox.datasetIndex : i);
                        if (meta) {
                            meta.hidden = !meta.hidden;
                            chart.update();
                            return;
                        }
                    }
                }
            }

            // For data point clicks
            const elements = chart.getElementsAtEventForMode(
                syntheticEvent,
                chart.options.interaction?.mode || 'nearest',
                chart.options.interaction || { intersect: false },
                false
            );
            // console.log('Elements found at click:', elements);

            // Try the internal Chart.js event handler
            chart._handleEvent(syntheticEvent);

            // Try triggering click handlers manually
            if (chart.options.onClick) {
                chart.options.onClick(syntheticEvent, elements, chart);
            }
        }
    }

    // console.log("sending ready");
    self.postMessage("");

    self.onmessage = (async event => {
        try {
            // Handle mouse events from main thread
            if (event.data && event.data.type === 'mouse-event') {
                const { eventType, x, y, chartId, computedStyles } = event.data;
                handleChartMouseEvent(eventType, x, y, chartId, computedStyles);
                return;
            }

            let [transaction, data] = event.data ?? [];
            // console.log(transaction, data);

            let {
                canvas, width, height,
                obj, mutate, plugins, defaults, id, // render()
                updated, animate                     // update()
            } = (data ?? {});

            if (obj) {
                if (defaults != null || defaults != undefined) {
                    defaults = eval(defaults);
                }

                if (plugins != null || plugins != undefined) {
                    obj.plugins = eval(plugins);
                }

                console.debug('Before mutate:', obj);
                if (mutate) {
                    obj = window.mutate_chart_object(obj)
                }
                console.debug('After mutate:', obj);
            }

            if (obj) obj = derationalize(obj);
            if (updated) updated = derationalize(updated);

            if (!updated) {
                console.log('New Chart');

                canvas.width = width;
                canvas.height = height;

                const chart = new Chart(canvas, obj);

                // Store chart instance for mouse event handling
                if (id) {
                    chartInstances.set(id, chart);
                }

                // Ensure animations work properly
                chart.resize();

                // Force an initial animation if configured
                if (obj.options?.animation !== false) {
                    chart.update('active');
                }
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
    });
})