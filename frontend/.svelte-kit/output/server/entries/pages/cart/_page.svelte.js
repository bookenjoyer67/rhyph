import { g as getContext, e as escape_html, b as attr, c as stringify, d as derived, f as store_get, u as unsubscribe_stores } from "../../../chunks/index.js";
import "clsx";
import "@sveltejs/kit/internal";
import "../../../chunks/exports.js";
import "../../../chunks/utils2.js";
import "@sveltejs/kit/internal/server";
import "../../../chunks/root.js";
import "../../../chunks/state.svelte.js";
const getStores = () => {
  const stores$1 = getContext("__svelte__");
  return {
    /** @type {typeof page} */
    page: {
      subscribe: stores$1.page.subscribe
    },
    /** @type {typeof navigating} */
    navigating: {
      subscribe: stores$1.navigating.subscribe
    },
    /** @type {typeof updated} */
    updated: stores$1.updated
  };
};
const page = {
  subscribe(fn) {
    const store = getStores().page;
    return store.subscribe(fn);
  }
};
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    var $$store_subs;
    const org = derived(() => store_get($$store_subs ??= {}, "$page", page).url.searchParams.get("org") || "default");
    const eventSlug = derived(() => store_get($$store_subs ??= {}, "$page", page).url.searchParams.get("event") || "");
    let positions = [];
    const positionCount = derived(() => positions.length);
    $$renderer2.push(`<div style="min-height:100vh;background:#111;color:#eee"><div style="max-width:800px;margin:0 auto;padding:48px 24px"><div style="display:flex;align-items:center;justify-content:space-between;margin-bottom:32px"><h1 style="font-size:2rem;font-weight:800;color:#fff;margin:0">Your Cart `);
    if (positionCount() > 0) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<span style="font-size:1rem;font-weight:400;color:#888">(${escape_html(positionCount())} ${escape_html(positionCount() === 1 ? "item" : "items")})</span>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></h1> `);
    if (eventSlug()) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<a${attr("href", `/events/${stringify(org())}/${stringify(eventSlug())}`)} style="color:#00E676;text-decoration:none;font-size:.95rem">← Back to event</a>`);
    } else {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--></div> `);
    {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<p style="color:#aaa;text-align:center;padding:60px;font-size:1.1rem">Loading cart...</p>`);
    }
    $$renderer2.push(`<!--]--></div></div>`);
    if ($$store_subs) unsubscribe_stores($$store_subs);
  });
}
export {
  _page as default
};
