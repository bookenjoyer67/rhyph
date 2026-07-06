import "clsx";
import "@sveltejs/kit/internal";
import "../../../../chunks/exports.js";
import "../../../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../../../chunks/root.js";
import "../../../../chunks/state.svelte.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    $$renderer2.push(`<div style="min-height:100vh;background:#111;color:#eee"><div style="max-width:800px;margin:0 auto;padding:48px 24px">`);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div style="display:flex;align-items:center;justify-content:center;min-height:60vh"><p style="color:#aaa;font-size:1.2rem">Loading order...</p></div>`);
    }
    $$renderer2.push(`<!--]--></div></div>`);
  });
}
export {
  _page as default
};
