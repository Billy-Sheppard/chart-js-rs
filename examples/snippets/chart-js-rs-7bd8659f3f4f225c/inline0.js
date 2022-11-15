export function render_chart(v, id, mutate) {
    console.debug(v);

    let obj;
    if (mutate) {
        obj = mutate_chart_object(v)
    }
    else {
        obj = v
    };

    console.debug(obj);

    const ctx = document.getElementById(id);
    var chart = new Chart(ctx, obj);
}