import { h as head } from "../../../chunks/index.js";
function _layout_($$renderer, $$props) {
  let { children } = $$props;
  head("1t7vkw9", $$renderer, ($$renderer2) => {
    $$renderer2.push(`<meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no, viewport-fit=cover"/> <meta name="apple-mobile-web-app-capable" content="yes"/> <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent"/> <meta name="theme-color" content="#111111"/>`);
  });
  children($$renderer);
  $$renderer.push(`<!---->`);
}
export {
  _layout_ as default
};
