import { b as attr, a as attr_style, e as escape_html, s as stringify } from "../../../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../../../chunks/exports.js";
import "../../../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../../../chunks/root.js";
import "../../../../chunks/state.svelte.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let listId = "";
    let loading = false;
    $$renderer2.push(`<div style="padding:32px;max-width:700px;margin:0 auto"><h1 style="font-size:1.6rem;color:#eee;margin:0 0 24px">Checkin Dashboard</h1> <form style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px;margin-bottom:24px;display:flex;gap:12px;align-items:end"><label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem;flex:1">Checkin List ID <input type="text"${attr("value", listId)} required="" style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none"/></label> <button type="submit"${attr("disabled", loading, true)}${attr_style(`background:#7c5ce7;color:#fff;border:none;padding:10px 24px;border-radius:6px;cursor:pointer;font-size:.95rem;white-space:nowrap;opacity:${stringify(1)}`)}>${escape_html("Fetch Stats")}</button></form> `);
    {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<p style="color:#888;text-align:center;padding:40px">Enter a checkin list ID to view stats.</p>`);
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
export {
  _page as default
};
