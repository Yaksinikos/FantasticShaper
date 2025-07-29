<script lang="ts">
  import { getCurrentWindow } from "@tauri-apps/api/window";

  import "./style.css";
  import { invoke } from "@tauri-apps/api/core";

  // when using `"withGlobalTauri": true`, you may use
  // const { getCurrentWindow } = window.__TAURI__.window;
  const appWindow = getCurrentWindow();
  document
    .getElementById("titlebar-minimize")
    ?.addEventListener("click", () => appWindow.minimize());
  // document
  //   .getElementById("titlebar-maximize")
  //   ?.addEventListener("click", () => appWindow.toggleMaximize());
  document
    .getElementById("titlebar-close")
    ?.addEventListener("click", () => appWindow.close());

  // Configuration
  let p_interface_value: string;
  let p_download_use: boolean;
  let p_download_value: number;
  let p_upload_use: boolean;
  let p_upload_value: number;

  const file = new File(
    ["/etc/systemd/wondershaper.conf"],
    "/etc/systemd/wondershaper.conf",
    { type: "file" }
  );
  const dataTransfer = new DataTransfer();
  dataTransfer.items.add(file);
  const fileList = dataTransfer.files;

  let p_ruleset_file: FileList | null = fileList;
  let p_ruleset_file_input: HTMLInputElement;
  let p_ruleset_file_path_string: HTMLElement;

  // UI Interface
  let v_interface_selector_a = "unset";
  let v_interface_selector_b = "none";

  function fn_ui_toggle_interface_selection_visibility() {
    let v_temp = v_interface_selector_a;
    v_interface_selector_a = v_interface_selector_b;
    v_interface_selector_b = v_temp;
  }

  // UI Download

  // UI File Ruleset
  function fn_ui_set_ruleset_file() {
    p_ruleset_file_path_string.innerText =
      p_ruleset_file?.item(0)?.name ?? "/etc/systemd/wondershaper.conf";
  }

  /*
  Update UI based on current WonderShaper Status
  */
  function fn_ui_update() {
    p_ruleset_file_path_string.innerHTML =
      p_ruleset_file?.item(0)?.name ?? "/etc/systemd/wondershaper.conf";
  }

  async function fn_ui_update_wondershaper_version() {
    let ressa: string = await invoke("fn_get_wondershaper_version", {});
    document.getElementById("id_wondershaper_version")?.setHTMLUnsafe(ressa);
  }

  /*
  Apply Custom Set Variables to WonderShaper
  */
  function fn_apply_set_rules() {
    document.getElementById("id_button_fireup")?.setAttribute("disabled", "");
    console.log("Hellow");
    invoke("fn_lib_cmd_fireup", {});
  }
</script>

<main style="background: var(--bg-color); height: 75vh; padding-top: 25vh;">
  <!-- Svelte Main -->
  <div>
    <!-- Main Content-->
    <div>
      <!-- Main Content Container -->
      <div style="margin: 25px">
        <!-- Main Customization Container -->
        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <!-- Main Customization Row -->
          <div
            style="display: flex; justify-content: right; width: 30%; color: var(--text-color); "
          >
            Interface:
          </div>
          <div>
            <div style="display: {v_interface_selector_a}; width: 70%;">
              <!-- Line -->
              <select
                id="adapter"
                bind:value={p_interface_value}
                style="color: var(--text-color);"
              >
                <option class="option_style" value="lo">lo (Loopback) </option>
                <option class="option_style" value="eth0" selected
                  >eth0 (Legacy Ethernet)
                </option>
                <option class="option_style" value="enp0s31f6"
                  >enp0s31f6 (Ethernet)
                </option>
                <option class="option_style" value="wlan0"
                  >wlan0 (Legacy Wireless Local Area Network)
                </option>
                <option class="option_style" value="wlp82s0"
                  >wlp82s0 (Wireless Fidelity - WiFi)
                </option>
                <option class="option_style" value="ppp0"
                  >ppp0 (Point to Point Protocol Network)
                </option>
                <option class="option_style" value="vboxnet0"
                  >vboxnet0 (Virtual Box Network)
                </option>
                <option class="option_style" value="vmnet1"
                  >vmnet1 (Virtual Machine Network Bridge Mode)
                </option>
                <option class="option_style" value="vmnet8"
                  >vmnet8 (Virtual Machine Network NAT Mode)
                </option>
              </select>
              <button
                class="button_style"
                style="color: var(--text-color)"
                onclick={(event) =>
                  fn_ui_toggle_interface_selection_visibility()}
                >&#8668;
              </button>
            </div>
            <div style="display: {v_interface_selector_b}; left: 70%;">
              <button
                class="button_style"
                onclick={(event) =>
                  fn_ui_toggle_interface_selection_visibility()}
                >&#8669;
              </button>
              <input placeholder="adapter_interface" />
            </div>
          </div>
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 30%; color: var(--text-color); "
          >
            Download:
          </div>
          <div>
            <input
              class="input_bps"
              id="down"
              type="number"
              min="0"
              bind:value={p_download_value}
              placeholder="... Kbps"
              oninput={(e: Event) => {
                if (!e.target) {
                  return;
                }
                if (e.target instanceof HTMLInputElement) {
                  if (e.target.value) {
                    let temp_d: number = parseInt(e.target.value);
                    if (temp_d > 0 && temp_d < 100) {
                      // script_ts.fn_set_download_limit(
                      // Math.max(0, Math.min(100, temp_d)),
                      // );
                    }
                  }
                }
              }}
            />
            <label class="input_slider_switch">
              <input type="checkbox" checked />
              <span class="input_slider"></span>
            </label>
          </div>
          <div style="color: green;">UNLIMITED</div>
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 30%; color: var(--text-color);"
          >
            Upload:
          </div>
          <div>
            <input
              class="input_bps"
              id="v_upload"
              type="number"
              min="0"
              bind:value={p_upload_value}
              placeholder="... Kbps"
              oninput={(e: Event) => {
                if (!e.target) {
                  return;
                }
                if (e.target instanceof HTMLInputElement) {
                  if (e.target.value) {
                    let temp_u: number = parseInt(e.target.value);
                    if (temp_u > 0 && temp_u < 100) {
                      // script_ts.fn_set_upload_limit(
                      // Math.max(0, Math.min(100, temp_u)),
                      // );
                    }
                  }
                }
              }}
            />
            <label class="input_slider_switch">
              <input type="checkbox" />
              <span class="input_slider"></span>
            </label>
          </div>
          <div style="color: green;">UNLIMITED</div>
        </div>
        <div style="display: flex; justify-content: right; margin-right: 35px">
          <!-- Main Control Container -->
          <button
            id="id_button_fireup"
            class="button_style"
            style="margin-right: 18px;"
            onclick={(e: Event) => fn_apply_set_rules()}>Apply</button
          >
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 30%; color: var(--text-color);"
          >
            Ruleset:
          </div>
          <div>
            <input
              bind:files={p_ruleset_file}
              bind:this={p_ruleset_file_input}
              id="id_ruleset_selection"
              type="file"
              accept=".conf"
              hidden
              onchange={(e: Event) => {
                fn_ui_update();
              }}
            />
            <button
              class="button_style"
              onclick={(e: Event) => {
                document.getElementById("id_ruleset_selection")?.click();
                return;
              }}
            >
              Select File
            </button>
            <button
              class="button_style"
              style="color: var(--text-color);"
              onclick={(e: Event) => {
                p_ruleset_file_input.value = "";
                p_ruleset_file = null;
                fn_ui_update();
              }}>&#x27F2;</button
            >
          </div>
          <button
            id="id_button_fireup"
            class="button_style"
            style="margin-left: auto;"
            onclick={(e: Event) => fn_apply_set_rules()}
            >Use Ruleset
          </button>
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 30%; color: var(--text-color);"
          >
            Current Ruleset:
          </div>
          <div
            bind:this={p_ruleset_file_path_string}
            id="id_selected_file_path"
            style="text-decoration-line: underline; font-style: italic; color: var(--text-color);     
            user-select: auto;
            -webkit-user-select: auto; /* Chrome, Safari, Opera */
            -moz-user-select: auto; /* Firefox */
            -ms-user-select: auto;"
          >
            /etc/systemd/wondershaper.conf
          </div>
        </div>
      </div>
    </div>
  </div>

  <div
    style="background-color: var(--subtle-color); position: fixed; bottom: 0px; width: 100%; display: flex; align-items: center; gap: 3px;"
  >
    <!-- Main Bottom Bar -->
    <div id="id_status_dot" class="dot" style="background-color: red;"></div>
    <div>Statuc</div>
    <button>dev_refresh</button>
    <div
      style="margin-right: 0; margin-left: auto;"
      id="id_wondershaper_version"
    >
      Installed version
    </div>
    <button
      style="visibility: show;"
      onclick={(e: Event) => fn_ui_update_wondershaper_version()}
      >dev_refresh</button
    >
  </div>
</main>

<style>
  .input_slider_switch {
    position: relative;
    display: inline-block;
    width: 60px;
    height: 34px;
  }

  .input_slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: var(--subtle-color);
    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  .input_slider:before {
    position: absolute;
    content: "";
    height: 26px;
    width: 26px;
    left: 4px;
    bottom: 4px;
    background-color: white;
    -webkit-transition: 0.4s;
    transition: 0.4s;
  }

  input:checked + .input_slider {
    background-color: var(--border-color);
  }

  input:focus + .input_slider {
    box-shadow: 0 0 1px #2196f3;
  }

  input:checked + .input_slider:before {
    -webkit-transform: translateX(26px);
    -ms-transform: translateX(26px);
    transform: translateX(26px);
  }

  /* From Uiverse.io by Jaareet */
  .input_bps {
    color: #fff;
    background-color: var(--input-color);
    box-shadow:
      0 0 0.4vw rgba(0, 0, 0, 0.5),
      0 0 0 0.15vw transparent;
    border-radius: 0.4vw;
    border: none;
    outline: none;
    padding: 0.4vw;
    max-width: 190px;
    transition: 0.4s;
  }

  .input_bps:hover {
    box-shadow: 0 0 0 0.15vw rgba(135, 207, 235, 0.186);
  }

  .input_bps:focus {
    box-shadow: 0 0 0 0.15vw skyblue;
  }

  .option_style {
    padding: 0 30px 0 10px;
    min-height: 40px;
    display: flex;
    align-items: center;
    background: #333;
    border-top: #222 solid 1px;
    position: absolute;
    top: 0;
    width: 100%;
    pointer-events: none;
    order: 2;
    z-index: 1;
    box-sizing: border-box;
    overflow: hidden;
    white-space: nowrap;
    color: var(--text-color);
  }

  .dot {
    height: 4vh;
    width: 4vh;
    background-color: #bbb;
    border-radius: 35%;
    display: inline-block;
  }
</style>
