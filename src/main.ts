import App from "./App.svelte";
import { mount } from "svelte";

const appElement = document.getElementById("app");

let appInstance;
if (appElement) {
  try {
    // Svelte 5 mount
    appInstance = mount(App, { target: appElement });
  } catch {
    // Svelte 4 fallback
    // @ts-expect-error Svelte 4 fallback
    appInstance = new App({ target: appElement });
  }
}

export default appInstance;
