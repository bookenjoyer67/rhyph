

export const index = 1;
let component_cache;
export const component = async () => component_cache ??= (await import('../entries/fallbacks/error.svelte.js')).default;
export const imports = ["_app/immutable/nodes/1.CtPhlvgT.js","_app/immutable/chunks/CKqRd2a3.js","_app/immutable/chunks/2aVgjNbk.js","_app/immutable/chunks/CYIH5RBy.js"];
export const stylesheets = [];
export const fonts = [];
