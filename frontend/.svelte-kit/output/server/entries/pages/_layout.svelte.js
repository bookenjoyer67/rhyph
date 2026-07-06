import { h as head, a as attr_style, d as derived } from "../../chunks/index.js";
import "@sveltejs/kit/internal";
import "../../chunks/exports.js";
import "../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../chunks/root.js";
import "../../chunks/state.svelte.js";
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { children } = $$props;
    const auth = derived(() => false);
    const linkStyle = "color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;transition:background .2s;font-size:.9rem";
    const primary = derived(() => "#7c5ce7");
    const accent = derived(() => "#00E676");
    const rootVars = derived(() => `--rhyph-primary:${primary()};--rhyph-accent:${accent()}${""}${""}${""}${""}`);
    head("12qhfyh", $$renderer2, ($$renderer3) => {
      {
        $$renderer3.push("<!--[-1-->");
      }
      $$renderer3.push(`<!--]-->`);
    });
    $$renderer2.push(`<header style="display:flex;align-items:center;justify-content:space-between;padding:12px 24px;background:var(--rhyph-bg, #0d0d1a);border-bottom:1px solid var(--rhyph-card-border, #2a2a4a)"><div style="display:flex;align-items:center;gap:24px">`);
    {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<a href="/" style="font-size:1.5rem;font-weight:700;color:var(--rhyph-primary, #7c5ce7);text-decoration:none">Rhyph</a>`);
    }
    $$renderer2.push(`<!--]--></div> <div style="display:flex;align-items:center;gap:12px">`);
    if (auth()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<a href="/admin/events"${attr_style(linkStyle)}>Events</a> <a href="/admin/orders"${attr_style(linkStyle)}>Orders</a> <a href="/admin/checkin"${attr_style(linkStyle)}>Checkin</a> <a href="/admin/devices"${attr_style(linkStyle)}>Devices</a> <a href="/scan"${attr_style(linkStyle)}>Scanner</a> <button style="background:var(--rhyph-primary, #7c5ce7);color:#fff;border:none;padding:8px 20px;border-radius:6px;cursor:pointer;font-size:.9rem;margin-left:8px">Logout</button>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<a href="/login" style="background:var(--rhyph-primary, #7c5ce7);color:#fff;text-decoration:none;padding:8px 20px;border-radius:6px;font-size:.9rem;font-weight:600">Login</a>`);
    }
    $$renderer2.push(`<!--]--></div></header> <main${attr_style(`min-height:calc(100vh - 61px);background:var(--rhyph-bg, #111);color:#eee;font-family:var(--rhyph-font, system-ui, sans-serif)${rootVars()}`)}>`);
    children($$renderer2);
    $$renderer2.push(`<!----></main>`);
  });
}
export {
  _layout as default
};
