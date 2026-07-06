

export const index = 2;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/pages/scan/_layout@.svelte.js')).default;
export const imports = ["_app/immutable/nodes/2.2B_wBnx1.js","_app/immutable/chunks/DJaCinOx.js","_app/immutable/chunks/BHhz15QU.js"];
export const stylesheets = [];
export const fonts = [];
