import { a as attr_style, d as derived } from "../../chunks/index.js";
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { children } = $$props;
    const auth = derived(() => false);
    const linkStyle = "color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;transition:background .2s;font-size:.9rem";
    $$renderer2.push(`<header style="display:flex;align-items:center;justify-content:space-between;padding:12px 24px;background:#0d0d1a;border-bottom:1px solid #2a2a4a"><div style="display:flex;align-items:center;gap:24px"><a href="/" style="font-size:1.5rem;font-weight:700;color:#7c5ce7;text-decoration:none">Rhyph</a></div> <div style="display:flex;align-items:center;gap:12px">`);
    if (auth()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<a href="/admin/events"${attr_style(linkStyle)}>Events</a> <a href="/admin/orders"${attr_style(linkStyle)}>Orders</a> <a href="/admin/checkin"${attr_style(linkStyle)}>Checkin</a> <a href="/admin/devices"${attr_style(linkStyle)}>Devices</a> <a href="/scan"${attr_style(linkStyle)}>Scanner</a> <button style="background:#7c5ce7;color:#fff;border:none;padding:8px 20px;border-radius:6px;cursor:pointer;font-size:.9rem;margin-left:8px">Logout</button>`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<a href="/login" style="background:#7c5ce7;color:#fff;text-decoration:none;padding:8px 20px;border-radius:6px;font-size:.9rem;font-weight:600">Login</a>`);
    }
    $$renderer2.push(`<!--]--></div></header> <main style="min-height:calc(100vh - 61px);background:#111;color:#eee">`);
    children($$renderer2);
    $$renderer2.push(`<!----></main>`);
  });
}
export {
  _layout as default
};
