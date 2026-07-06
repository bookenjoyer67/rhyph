import { b as attr, a as attr_style, e as escape_html, ab as ensure_array_like } from "../../../chunks/index.js";
function _page($$renderer, $$props) {
  $$renderer.component(($$renderer2) => {
    const storedKey = localStorage.getItem("rhyph_scanner_key") || "";
    let apiKey = storedKey;
    let apiKeyInput = "";
    const storedEvent = localStorage.getItem("rhyph_scanner_event") || "";
    let eventSlug = storedEvent;
    let checkinLists = [];
    let selectedListId = "";
    if (!apiKey) {
      $$renderer2.push("<!--[0-->");
      $$renderer2.push(`<div style="display:flex;align-items:center;justify-content:center;min-height:100vh;min-height:100dvh;background:#111;padding:24px"><form style="display:flex;flex-direction:column;gap:16px;width:100%;max-width:400px;padding:32px 24px;background:#1a1a2e;border-radius:16px;border:1px solid #2a2a4a"><h1 style="font-size:1.6rem;font-weight:700;color:#7c5ce7;text-align:center;margin:0">Rhyph Scanner</h1> <p style="color:#888;text-align:center;margin:0;font-size:.9rem">Enter your scanner API key to connect</p> <input type="password"${attr("value", apiKeyInput)} placeholder="Scanner API key" autocomplete="off" style="padding:14px 16px;border:1px solid #2a2a4a;border-radius:10px;background:#0d0d1a;color:#eee;font-size:1.05rem;outline:none;min-height:48px"/> <button type="submit"${attr("disabled", !apiKeyInput.trim(), true)}${attr_style(`background:#7c5ce7;color:#fff;border:none;padding:14px;border-radius:10px;cursor:pointer;font-size:1.1rem;font-weight:600;min-height:48px;opacity:${apiKeyInput.trim() ? "1" : "0.4"}`)}>Connect</button></form></div>`);
    } else {
      $$renderer2.push("<!--[1-->");
      $$renderer2.push(`<div style="display:flex;flex-direction:column;min-height:100vh;min-height:100dvh;background:#111;padding:16px;gap:16px"><div style="display:flex;align-items:center;justify-content:space-between"><h1 style="font-size:1.2rem;font-weight:700;color:#7c5ce7;margin:0">Rhyph Scanner</h1> <button style="background:transparent;color:#e74c3c;border:1px solid #e74c3c;padding:8px 16px;border-radius:8px;font-size:.85rem;cursor:pointer;min-height:48px">Disconnect</button></div> <div style="display:flex;flex-direction:column;gap:12px;flex:1"><label style="display:flex;flex-direction:column;gap:6px;color:#aaa;font-size:.85rem">Event slug <input type="text"${attr("value", eventSlug)} placeholder="e.g. summer-fest-2026" style="padding:12px 16px;border:1px solid #2a2a4a;border-radius:10px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none;min-height:48px"/></label> <label style="display:flex;flex-direction:column;gap:6px;color:#aaa;font-size:.85rem">Check-in list `);
      $$renderer2.select(
        {
          value: selectedListId,
          disabled: checkinLists.length === 0,
          style: "padding:12px 16px;border:1px solid #2a2a4a;border-radius:10px;background:#0d0d1a;color:#eee;font-size:1rem;outline:none;min-height:48px;appearance:none",
          onchange: () => {
          }
        },
        ($$renderer3) => {
          $$renderer3.option({ value: "" }, ($$renderer4) => {
            $$renderer4.push(`${escape_html("Select a list")}`);
          });
          $$renderer3.push(`<!--[-->`);
          const each_array = ensure_array_like(checkinLists);
          for (let $$index = 0, $$length = each_array.length; $$index < $$length; $$index++) {
            let list = each_array[$$index];
            $$renderer3.option({ value: list.id }, ($$renderer4) => {
              $$renderer4.push(`${escape_html(list.name)}`);
            });
          }
          $$renderer3.push(`<!--]-->`);
        }
      );
      $$renderer2.push(`</label> `);
      {
        $$renderer2.push("<!--[-1-->");
      }
      $$renderer2.push(`<!--]--></div></div>`);
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]--> `);
    {
      $$renderer2.push("<!--[-1-->");
    }
    $$renderer2.push(`<!--]-->`);
  });
}
export {
  _page as default
};
