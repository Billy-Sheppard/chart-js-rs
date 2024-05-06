export function render_chart(v, id, mutate) {
    console.debug('Before mutate:', v);

    let obj;
    if (mutate) {
        obj = mutate_chart_object(v)
    }
    else {
        obj = v
    };

    console.debug('After mutate:', obj);

    const ctx = document.getElementById(id);
    let chart = new Chart(ctx, obj);
}