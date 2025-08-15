<script lang="ts">
  import "./style.css";
  import { getCurrentWindow } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { VERSION } from "svelte/compiler";

  // This Tauri/Svelte GUI Version
  const gui_version: string = VERSION;
  // GUI State (GUI / Log-Page)
  const gui_state: number = 0;

  // Prerequisites Installed
  let tc_installed: boolean = false;
  let cat_installed: boolean = false;
  let modprobe_installed: boolean = false;
  let shell_installed: boolean = false;
  let ws_installed: boolean = false;

  // WonderShaper Version
  let ws_version: string = "";
  // WonderShaper default config
  let ws_conf: string = "";
  // WonderShaper default legacy config
  let ws_conflegacy: string = "";
  // WonderShaper deamon usage
  let ws_systemd: boolean = false;

  // GUI offered custom preset selection
  let gui_custom_preset_valid: boolean = false;
  let gui_custom_preset_file: string = "";
  let gui_custom_preset_timestamp: string = "";

  // Config Read In settings to use in commands
  let ws_config_dspeed: number = 0;
  let ws_config_uspeed: number = 0;
  let ws_config_iface: string = "";

  // GUI offered command customization
  let gui_dlimit: boolean = false;
  let gui_ulimit: boolean = false;
  let gui_custom_iface: boolean = false;

  // Limitation active or not
  let ws_active: boolean = false;

  /**
   * Tauri Setup
   */
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

  /**
   * GUI Initialization Functions
   */

  /**
   * GUI Process Functions
   */

  // Configuration
  let p_interface_value: string;
  let p_download_use: boolean;
  let p_download_value: number;
  let p_upload_use: boolean;
  let p_upload_value: number;

  const file = new File(
    ["/etc/systemd/wondershaper.conf"],
    "/etc/systemd/wondershaper.conf",
    { type: "file" },
  );
  const dataTransfer = new DataTransfer();
  dataTransfer.items.add(file);
  const fileList = dataTransfer.files;

  let p_ruleset_file: FileList | null = fileList;
  let p_ruleset_file_input: HTMLInputElement;
  let p_ruleset_file_path_string: HTMLInputElement;

  // UI Interface
  let v_interface_selector_a = "unset";
  let v_interface_selector_b = "none";

  function fn_check_installed(program: string): Boolean {
    return true;
  }

  /*
  Initial State Parser
  */
  async function fn_onMount_state_parse() {
    /*
    - Check if WonderShaper is installed
    - check if requeried tools are installed
    - Get WonderShaper Version
    - check if wondershaper service is active
    - Check if currently a limitation is applied

    - Get currently used Interface

    - set file selection to default preset file
    */
  }

  /*
   */
  function fn_ui_toggle_interface_selection_visibility() {
    let v_temp = v_interface_selector_a;
    v_interface_selector_a = v_interface_selector_b;
    v_interface_selector_b = v_temp;
  }

  /*
  Update UI based on current WonderShaper Status
  */
  function fn_ui_update_file_selected() {
    if (p_ruleset_file?.item(0)?.name.endsWith(".conf")) {
      p_ruleset_file_path_string.value =
        p_ruleset_file?.item(0)?.name ?? "/etc/systemd/wondershaper.conf";
    } else {
      p_ruleset_file_path_string.value = "corrupted selection";
    }
  }

  /*
   */
  async function fn_ui_update_wondershaper_version() {
    let ressa: string = await invoke("fn_get_wondershaper_version", {});
    document.getElementById("id_wondershaper_version")?.setHTMLUnsafe(ressa);
  }

  /*
   */
  function fn_ui_show_custom_interface_input(value: boolean) {
    if (value == true) {
      document
        .getElementById("id_ui_custom_interface")
        ?.classList.remove("hidden");
    } else {
      document
        .getElementById("id_ui_custom_interface")
        ?.classList.add("hidden");
    }
  }

  /*
   */
  async function fn_ui_update_active_interface() {
    let ressas: string = await invoke("fn_get_active_interface", {});
    // Update Selected Internet Interface
    p_interface_value = ressas;
  }

  /*
  Apply Custom Set Variables to WonderShaper
  */
  function fn_apply_set_rules() {
    document.getElementById("id_button_fireup")?.setAttribute("disabled", "");
    invoke("fn_lib_cmd_fireup", {});
  }

  onMount(async () => {
    await fn_ui_update_active_interface();
    await fn_ui_update_wondershaper_version();

    fn_ui_update_file_selected();
  });
</script>

<main style="background: var(--bg-color); height: 75vh; padding-top: 20vh;">
  <!-- Svelte Main -->

  <div>
    <!-- Main Content Container -->
    <div>
      <!-- Main Log Container -->
    </div>
    <div style="margin: 20px">
      <!-- Main Customization Container -->
      <div>
        <!-- Top Customization Container -->

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <!-- Main Customization Row -->
          <div
            style="display: flex; justify-content: right; width: 20vh; color: var(--text-color); "
          >
            Interface:
          </div>
          <div>
            <div style="width: 70%;">
              <!-- Line -->
              <select
                id="adapter"
                bind:value={p_interface_value}
                style="color: var(--text-color);"
                onchange={(e) => {
                  console.log("What is this?");
                  if (p_interface_value == "custom") {
                    fn_ui_show_custom_interface_input(true);
                  } else {
                    fn_ui_show_custom_interface_input(false);
                  }
                }}
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
                <option class="option_style" value="custom">custom</option>
              </select>
            </div>
          </div>
          <div id="id_ui_custom_interface" class="hidden">
            <input />
          </div>
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 20vh; color: var(--text-color); "
          >
            Download:
          </div>
          <div>
            <input
              class="input_bps"
              id="down"
              type="number"
              min="0"
              style="margin: 5px; align-items:center; gap: 3px;"
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
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 20vh; color: var(--text-color);"
          >
            Upload:
          </div>
          <div>
            <input
              class="input_bps"
              id="v_upload"
              type="number"
              min="0"
              style="margin: 5px; align-items:center; gap: 3px;"
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
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <button
            id="id_button_fireup"
            class="button_style"
            style="margin-left: auto;"
            onclick={(e: Event) => fn_apply_set_rules()}>&#x26A0 Limit</button
          >
        </div>
      </div>
      <hr />
      <div>
        <!-- Buttom Customization Container -->
        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 20vh; color: var(--text-color);"
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
                fn_ui_update_file_selected();
                document
                  .getElementById("id_selected_file_timestamp")
                  ?.setHTMLUnsafe(new Date().toLocaleString());
              }}
            />
            <button
              class="button_style"
              onclick={(e: Event) => {
                document.getElementById("id_ruleset_selection")?.click();
                return;
              }}
            >
              &#x1F4CE
            </button>
            <button
              class="button_style"
              style="color: var(--text-color);"
              onclick={(e: Event) => {
                // reset to default ruleset
                p_ruleset_file_input.value = "";
                p_ruleset_file = null;
                fn_ui_update_file_selected();
              }}>&#x27F2;</button
            >
          </div>
          <button
            id="id_button_fireup"
            class="button_style"
            style="margin-left: auto;"
            onclick={(e: Event) => fn_apply_set_rules()}
            >Read In Ruleset
          </button>
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 20vh; color: var(--text-color);"
          ></div>
          <input
            bind:this={p_ruleset_file_path_string}
            id="id_selected_file_path"
            style="text-decoration-line: underline; font-style: italic; color: var(--text-color);     
            user-select: auto;
            -webkit-user-select: auto; /* Chrome, Safari, Opera */
            -moz-user-select: auto; /* Firefox */
            -ms-user-select: auto;"
            value=""
          />
          <div id="id_selected_file_timestamp">(timestamp)</div>
        </div>

        <div style="display: flex; margin: 5px; align-items: center; gap: 3px;">
          <div
            style="display: flex; justify-content: right; width: 20vh; color: var(--text-color);"
          >
            Autostart:
          </div>
          <label class="input_slider_switch">
            <input type="checkbox" />
            <span class="input_slider"></span>
          </label>
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
    <button onclick={(e) => fn_ui_update_active_interface()}>dev_refresh</button
    >

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
