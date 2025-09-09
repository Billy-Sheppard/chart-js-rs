const wasm = await import('http://localhost:8080/chart_js_rs_example.js');
self.window.callbacks = wasm;
await wasm.default();

self.window.mutate_chart_object = function (v) {
    if (v.id == "bar") {
        v.options.scales.y1.ticks = {
            callback:
                function (value, _index, _values) {
                    return '$' + value.toFixed(2);
                }
        };
    }
    return v
};

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
    // console.log('DateTime constants available');
} else {
    console.error('DateTime constants not available');
}

// Now import the adapter
await import("https://cdn.jsdelivr.net/npm/chartjs-adapter-luxon@^1/dist/chartjs-adapter-luxon.umd.min.js");