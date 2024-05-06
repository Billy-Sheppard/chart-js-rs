export function get_chart(id) {
    return Chart.getChart(document.getElementById(id)).config._config
}

export function render_chart(v, id, mutate) {
    console.debug('Before mutate:', v);

    let obj;
    if (mutate) {
        obj = window.mutate_chart_object(v)
    }
    else {
        obj = v
    };

    console.debug('After mutate:', obj);

    const ctx = document.getElementById(id);
    let chart = new Chart(ctx, obj);
}

export function update_chart(updated, id, animate) {
    try {
        let chart = Chart.getChart(document.getElementById(id));
        chart.config._config.type = updated.type;
        chart.config._config.data = updated.data;
        chart.config._config.options = updated.options;

        console.debug('Updated chart:', chart);

        if (animate) {
            chart.update();
        } else {
            chart.update('none');
        }

        true
    }
    catch {
        false
    }
}