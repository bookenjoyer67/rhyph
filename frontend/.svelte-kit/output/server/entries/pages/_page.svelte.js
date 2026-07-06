import { b as attr, a as attr_style, e as escape_html, s as stringify } from "../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../chunks/exports.js";
import "../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../chunks/root.js";
import "../../chunks/state.svelte.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let email = "";
    let password = "";
    let loading = false;
    $$renderer2.push(`<div style="display:flex;align-items:center;justify-content:center;min-height:100vh;background:#111"><form style="display:flex;flex-direction:column;gap:16px;width:100%;max-width:400px;padding:40px;background:#1a1a2e;border-radius:12px;border:1px solid #2a2a4a"><h1 style="font-size:1.8rem;font-weight:700;color:#7c5ce7;text-align:center;margin:0">Rhyph Admin</h1> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">Email <input type="email"${attr("value", email)} required="" style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"/></label> <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">Password <input type="password"${attr("value", password)} required="" style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"/></label> <button type="submit"${attr("disabled", loading, true)}${attr_style(`background:#7c5ce7;color:#fff;border:none;padding:12px;border-radius:6px;cursor:pointer;font-size:1rem;font-weight:600;margin-top:8px;opacity:${stringify(1)}`)}>${escape_html("Login")}</button></form></div>`);
  });
}
export {
  _page as default
};
