import "clsx";
import "@sveltejs/kit/internal";
import "../../../../../chunks/exports.js";
import "../../../../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../../../../chunks/root.js";
import "../../../../../chunks/state.svelte.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    $$renderer2.push(`<div style="padding:32px;max-width:1000px;margin:0 auto"><div style="display:flex;align-items:center;gap:16px;margin-bottom:24px"><button style="background:transparent;color:#aaa;border:none;cursor:pointer;font-size:1.3rem;padding:0 4px">←</button> <h1 style="font-size:1.6rem;color:#eee;margin:0">`);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`Loading...`);
    }
    $$renderer2.push(`<!--]--></h1></div> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<p style="color:#aaa;text-align:center;padding:40px">Loading event...</p>`);
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
export {
  _page as default
};
