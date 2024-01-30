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