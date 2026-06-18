
    export function worxide_glue_url(crate_name) {
        // Cargo replaces hyphens with underscores in library output filenames,
        // so a crate named "my-app" produces "my_app.js". Strip a trailing
        // ".js" so callers may pass either form.
        const file = crate_name.replace(/-/g, "_").replace(/\.js$/, "");
        return new URL("../../" + file + ".js", import.meta.url).href;
    }
    export function worxide_glue_url_from_path(path) {
        // Resolve against the document base; do NOT mangle the string.
        return new URL(path, document.baseURI).href;
    }
    export function worxide_app_js_path() {
        // Optional consumer-set global, e.g. one line in HTML:
        //   globalThis.app_js_path = "my_app.js";   // or "/static/my_app.js"
        // Read from globalThis so it is safe to call from any context; returns
        // null when unset / not a non-empty string so wasm-bindgen maps it to
        // `None`.
        const p = globalThis.app_js_path;
        return (typeof p === "string" && p.length > 0) ? p : null;
    }
