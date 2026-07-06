import { a as attr_style, d as derived } from "../../chunks/index.js";
function _layout($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    let { children } = $$props;
    const auth = derived(() => false);
    const navLink = (path) => `color:#aaa;text-decoration:none;padding:8px 16px;border-radius:6px;transition:background .2s`;
    const navActive = (path) => {
      if (typeof window !== "undefined" && window.location.pathname.startsWith(path)) {
        return "background:#333;color:#fff";
      }
      return "";
    };
    if (auth()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<nav style="display:flex;align-items:center;justify-content:space-between;padding:12px 24px;background:#0d0d1a;border-bottom:1px solid #2a2a4a"><div style="display:flex;align-items:center;gap:24px"><a href="/" style="font-size:1.5rem;font-weight:700;color:#7c5ce7;text-decoration:none">Rhyph</a> <a href="/admin/events"${attr_style(navLink() + navActive("/admin/events"))}>Events</a> <a href="/admin/orders"${attr_style(navLink() + navActive("/admin/orders"))}>Orders</a> <a href="/admin/checkin"${attr_style(navLink() + navActive("/admin/checkin"))}>Checkin</a> <a href="/admin/devices"${attr_style(navLink() + navActive("/admin/devices"))}>Devices</a></div> <button style="background:#7c5ce7;color:#fff;border:none;padding:8px 20px;border-radius:6px;cursor:pointer;font-size:.9rem">Logout</button></nav>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> <main style="min-height:calc(100vh - 61px);background:#111;color:#eee">`);
    if (auth()) {
      $$renderer2.push("<!--[0-->");
      children($$renderer2);
      $$renderer2.push(`<!---->`);
    } else {
      $$renderer2.push("<!--[-1-->");
      $$renderer2.push(`<div style="display:flex;align-items:center;justify-content:center;min-height:100vh;background:#111"><form style="display:flex;flex-direction:column;gap:16px;width:100%;max-width:400px;padding:40px;background:#1a1a2e;border-radius:12px;border:1px solid #2a2a4a"><h1 style="font-size:1.8rem;font-weight:700;color:#7c5ce7;text-align:center;margin:0">Rhyph Admin</h1> <input type="email" name="email" placeholder="Email" required="" style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"/> <input type="password" name="password" placeholder="Password" required="" style="padding:12px;border:1px solid #2a2a4a;border-radius:6px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none"/> <button type="submit" style="background:#7c5ce7;color:#fff;border:none;padding:12px;border-radius:6px;cursor:pointer;font-size:1rem;font-weight:600;margin-top:8px">Login</button></form></div>`);
    }
    $$renderer2.push(`<!--]--></main>`);
  });
}
export {
  _layout as default
};
