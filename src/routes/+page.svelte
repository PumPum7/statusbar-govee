<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import DeviceList from "../components/DeviceList.svelte";
  import type { Device, DeviceState } from "../types";

  let devices: Device[] = $state([]);
  let error: string | null = $state(null);
  let deviceStates: Record<string, DeviceState> = $state({});

  onMount(() => {
    invoke("init");
    handleRefresh();
  });

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
      console.error("Failed to get device state:", e);
      return null;
    }
  }

  async function refreshDeviceStates() {
    for (const device of devices) {
      deviceStates[device.device] = await getDeviceState(device.device, device.sku) as DeviceState;
    }
    console.log(deviceStates);
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
      console.log("test", deviceStates);
    } catch (e) {
      error = e as string;
    }
  }
</script>

<main class="menubar-container">
  {#if error}
    <div class="error">{error}</div>
  {/if}

  <DeviceList
    {devices}
    {deviceStates}
    onRefresh={handleRefresh}
    onTogglePower={togglePower}
    onChangeCapabilityValue={changeCapabilityValue}
  />
</main>

<style>
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
