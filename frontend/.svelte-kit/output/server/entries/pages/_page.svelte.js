import "clsx";
import "@sveltejs/kit/internal";
import "../../chunks/exports.js";
import "../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../chunks/root.js";
import "../../chunks/state.svelte.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    $$renderer2.push(`<div style="display:flex;flex-direction:column;align-items:center;justify-content:center;min-height:calc(100vh - 61px);background:#111;padding:48px 24px;text-align:center"><h1 style="font-size:3.5rem;font-weight:800;color:#fff;margin:0 0 16px;letter-spacing:-1px">Rhyph</h1> <p style="font-size:1.3rem;color:#888;margin:0 0 8px;max-width:500px">Self-hosted ticketing for venues and bands.</p> <p style="font-size:1rem;color:#666;margin:0 0 40px;max-width:450px">No third party. No per-ticket fees. Your events, your data.</p> <div style="display:flex;gap:16px;flex-wrap:wrap;justify-content:center"><button style="background:#7c5ce7;color:#fff;border:none;padding:14px 36px;border-radius:10px;cursor:pointer;font-size:1.1rem;font-weight:700">Admin Login</button> <button style="background:transparent;color:#7c5ce7;border:2px solid #7c5ce7;padding:14px 36px;border-radius:10px;cursor:pointer;font-size:1.1rem;font-weight:700">Scanner</button></div></div>`);
  });
}
export {
  _page as default
};
