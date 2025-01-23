<script lang="ts">
  import type { Device, DeviceCapabilityInstance, DeviceCapabilityValue, DeviceState, DeviceCapabilityType, SceneOption } from '../types';
  import { numberToRGB } from '../types';
  import { invoke } from '@tauri-apps/api/core';
  import StatusIndicator from './StatusIndicator.svelte';
  import PowerControl from './PowerControl.svelte';
  import BrightnessControl from './BrightnessControl.svelte';
  import ColorControl from './ColorControl.svelte';
  import ScenesControl from './ScenesControl.svelte';

  export let device: Device;
  export let deviceState: DeviceState;
  export let onTogglePower: (device: string, sku: string, currentState: boolean) => Promise<void>;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: DeviceCapabilityType, value: DeviceCapabilityValue, instance: DeviceCapabilityInstance) => Promise<void>;

  let isPowerLoading = false;
  let error: string | null = null;
  let lightScenes: SceneOption[] = [];
  let diyScenes: SceneOption[] = [];
  let isLoadingScenes = false;
  let showLightScenes = false;
  let showDiyScenes = false;

  async function loadScenes() {
    if (device.type !== 'light') return;
    try {
      isLoadingScenes = true;
      const [lightSceneResult, diySceneResult] = await Promise.all([
        invoke('get_light_scenes', { device: device.device, sku: device.sku }),
        invoke('get_diy_scenes', { device: device.device, sku: device.sku })
      ]);
      lightScenes = lightSceneResult as SceneOption[];
      diyScenes = diySceneResult as SceneOption[];
    } catch (e) {
      error = e as string;
    } finally {
      isLoadingScenes = false;
    }
  }

  async function handlePowerToggle() {
    if (isPowerLoading) return;
    
    try {
      isPowerLoading = true;
      error = null;
      await onTogglePower(device.device, device.sku, status?.powerState || false);
    } catch (e) {
      error = e as string;
    } finally {
      isPowerLoading = false;
    }
  }

  function getDeviceIcon(device_type: string) {
    switch (device_type) {
      case 'thermometer': return '🌡️';
      case 'light': return '💡';
      default: return '📱';
    }
  }

  function getDeviceStatus(state: DeviceState) {
    if (!state) return null;

    const getCapabilityValue = <T extends DeviceCapabilityValue>(instance: DeviceCapabilityInstance): T | undefined => {
      const capability = state.capabilities.find(c => c.instance === instance);
      return capability?.state.value as T | undefined;
    };

    return {
      powerState: getCapabilityValue<boolean>('powerSwitch'),
      temperature: getCapabilityValue<number>('sensorTemperature'),
      humidity: getCapabilityValue<{ currentHumidity: number }>('sensorHumidity')?.currentHumidity,
      brightness: getCapabilityValue<number>('brightness'),
      color: getCapabilityValue<number>('colorRgb'),
    };
  }

  $: currentColor = status?.color ? numberToRGB(status.color) : null;

  $: status = getDeviceStatus(deviceState);

  $: {
    if (deviceState && device.type === 'light') {
      loadScenes();
    }
  }
</script>

<div class="device-card" data-type={device.type}>
  <div class="device-header">
    <span class="device-icon">{getDeviceIcon(device.type)}</span>
    <span class="device-name">{device.deviceName || 'Unnamed Device'}</span>
  </div>
  
  {#if deviceState}
    <div class="device-status">
      {#if device.type === 'thermometer'}
        <StatusIndicator 
          label="Temperature" 
          value={`${status?.temperature}°C`}
          icon="🌡️"
        />
        {#if status?.humidity !== undefined}
          <StatusIndicator 
            label="Humidity" 
            value={`${status.humidity}%`}
            icon="💧"
          />
        {/if}
      {:else if device.type === 'light'}
        <PowerControl
          {device}
          powerState={status?.powerState || false}
          {isPowerLoading}
          {onTogglePower}
        />
        {#if status?.powerState}
          {#if status?.brightness !== undefined}
            <BrightnessControl
              {device}
              brightness={status.brightness}
              {onChangeCapabilityValue}
            />
          {/if}

          {#if deviceState.capabilities.some(c => c.instance === 'colorRgb')}
            <ColorControl
              {device}
              currentColor={currentColor}
              {onChangeCapabilityValue}
            />
            <ScenesControl
              {device}
              {lightScenes}
              {diyScenes}
              {showLightScenes}
              {showDiyScenes}
              {onChangeCapabilityValue}
            />
          {/if}
        {/if}
      {:else}
        <StatusIndicator 
          label="Status" 
          value="Connected"
          icon="✅"
        />
      {/if}
    </div>
  {:else}
    <div class="loading">Loading...</div>
  {/if}
</div>

<style>
  .device-card {
    background: rgba(255, 255, 255, 0.1);
    padding: 1rem;
    border-radius: 12px;
    backdrop-filter: blur(10px);
    border: 1px solid rgba(255, 255, 255, 0.1);
    transition: transform 0.2s, box-shadow 0.2s;
  }

  .device-card:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  }

  .device-header {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    margin-bottom: 1rem;
  }

  .device-icon {
    font-size: 1.5rem;
  }

  .device-name {
    font-weight: 500;
    color: #ffffff;
  }

  .device-status {
    display: grid;
    gap: 0.75rem;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .loading {
    color: #999;
    font-style: italic;
    text-align: center;
    padding: 1rem 0;
  }

  [data-type="thermometer"] {
    background: linear-gradient(135deg, rgba(59, 130, 246, 0.2), rgba(37, 99, 235, 0.2));
  }

  [data-type="light"] {
    background: linear-gradient(135deg, rgba(250, 204, 21, 0.2), rgba(234, 179, 8, 0.2));
  }
</style>
