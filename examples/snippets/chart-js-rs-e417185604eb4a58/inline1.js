export function get_chart(id) {
    return Chart.getChart(document.getElementById(id)).config._config
}