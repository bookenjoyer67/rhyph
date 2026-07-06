import { b as attr, a as attr_style, e as escape_html, c as stringify } from "../../../../chunks/index.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let orgSlug = "default";
    let eventSlug = "";
    let code = "";
    let loading = false;
    $$renderer2.push(`<div style="padding:32px;max-width:800px;margin:0 auto"><h1 style="font-size:1.6rem;color:#eee;margin:0 0 24px">Order Lookup</h1> <form style="background:#1a1a2e;border:1px solid #2a2a4a;border-radius:12px;padding:24px;margin-bottom:24px"><div style="display:grid;grid-template-columns:1fr 1fr 1fr;gap:12px;align-items:end"><label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">Organizer Slug <input type="text"${attr("value", orgSlug)} style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none"/></label> <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">Event Slug * <input type="text"${attr("value", eventSlug)} required="" style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none"/></label> <label style="display:flex;flex-direction:column;gap:4px;color:#aaa;font-size:.85rem">Order Code * <input type="text"${attr("value", code)} required="" style="padding:10px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;outline:none"/></label></div> <button type="submit"${attr("disabled", loading, true)}${attr_style(`margin-top:16px;background:#7c5ce7;color:#fff;border:none;padding:10px 28px;border-radius:6px;cursor:pointer;font-size:.95rem;opacity:${stringify(1)}`)}>${escape_html("Lookup")}</button></form> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div>`);
  });
}
export {
  _page as default
};
