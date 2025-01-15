<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  
  import DeviceList from "../components/DeviceList.svelte";
  import type { Device, DeviceState } from "../types";

  let devices: Device[] = $state([]);
  let error: string | null = $state(null);
  let deviceStates: Record<string, DeviceState> = $state({});
  let apiKey: string = $state("");
  let hasApiKey: boolean = $state(false);

  onMount(async () => {
    invoke("init");
    let apiKey = await invoke("get_api_key");
    hasApiKey = !!apiKey;
    if (hasApiKey) {
      handleRefresh();
    }
  });

  async function handleSetApiKey() {
    try {
      await invoke("set_api_key", { apiKey });
      hasApiKey = true;
      handleRefresh();
    } catch (e) {
      error = e as string;
    }
  }

  async function getDevices() {
    try {
      devices = await invoke("get_devices");
      error = null;
    } catch (e) {
      error = e as string;
    }
  }

  async function getDeviceState(device: string, sku: string) {
    try {
      const state = await invoke("get_device_state", { device, sku });
      return state;
    } catch (e) {
      error = e as string;
      return null;
    }
  }

  async function refreshDeviceStates() {
    for (const device of devices) {
      deviceStates[device.device] = await getDeviceState(device.device, device.sku) as DeviceState;
    }
  }

  async function handleRefresh() {
    await getDevices();
    await refreshDeviceStates();
  }

  async function togglePower(device: string, sku: string, currentState: boolean) {
    changeCapabilityValue(device, sku, 'devices.capabilities.on_off', !currentState ? 1 : 0, 'powerSwitch');
  }

  async function changeCapabilityValue(device: string, sku: string, capabilityType: string, value: any, instance: string) {
    try {
      await invoke('change_capability_value', { device, sku, capabilityType, value, instance });
      await refreshDeviceStates();
    } catch (e) {
      error = e as string;
    }
  }
</script>

<main class="menubar-container">
  {#if error}
    <div class="error">{error}</div>
  {/if}

  {#if !hasApiKey}
    <div class="api-key-form">
      <h2>Welcome to Govee Statusbar</h2>
      <p>Please enter your Govee API key to get started:</p>
      <input
        type="text"
        bind:value={apiKey}
        placeholder="Enter your Govee API key"
      />
      <button onclick={handleSetApiKey}>Set API Key</button>
      <p class="help-text">
        You can find your API key in the Govee Home app under Profile > Settings > About Us > Apply for API Key
      </p>
    </div>
  {:else}

    <DeviceList
      {devices}
      {deviceStates}
      onRefresh={handleRefresh}
      onTogglePower={togglePower}
      onChangeCapabilityValue={changeCapabilityValue}
    />
  {/if}
</main>

<style>
  .api-key-form {
    padding: 2rem;
    text-align: center;
  }

  .api-key-form h2 {
    margin: 0 0 1rem 0;
    font-size: 1.5rem;
  }

  .api-key-form input {
    width: 100%;
    padding: 0.5rem;
    margin: 1rem 0;
    border: 1px solid #333;
    border-radius: 6px;
    background: #2a2a2a;
    color: white;
  }

  .api-key-form button {
    background: #4a4a4a;
    border: none;
    color: #ffffff;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background-color 0.2s;
    width: 100%;
  }

  .api-key-form button:hover {
    background: #5a5a5a;
  }

  .help-text {
    font-size: 0.8rem;
    color: #999;
    margin-top: 1rem;
  }

  .menubar-container {
    background-color: #1a1a1a;
    color: #ffffff;
    border-radius: 12px;
    overflow: hidden;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.25);
    min-height: 100vh;
  }

  .error {
    background: rgba(239, 68, 68, 0.2);
    color: #fecaca;
    padding: 1rem;
    margin: 1rem;
    border-radius: 8px;
    border: 1px solid rgba(239, 68, 68, 0.3);
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background: transparent;
    font-family: system-ui, -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  }

  :global(button) {
    font-family: inherit;
  }
</style>
