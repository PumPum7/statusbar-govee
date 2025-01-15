<script lang="ts">
  import type { Device, DeviceState } from '../types';
  import StatusIndicator from './StatusIndicator.svelte';

  export let device: Device;
  export let deviceState: DeviceState;
  export let onTogglePower: (device: string, sku: string, currentState: boolean) => Promise<void>;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: string, value: number, instance: string) => Promise<void>;

  function getDeviceIcon(device_type: string) {
    switch (device_type) {
      case 'thermometer': return '🌡️';
      case 'light': return '💡';
      default: return '📱';
    }
  }

  function getDeviceStatus(state: DeviceState) {
    if (!state) return null;

    const powerState = state.capabilities.find(c => c.instance === 'powerSwitch')?.state.value;
    const temperature = state.capabilities.find(c => c.instance === 'sensorTemperature')?.state.value;
    const humidity = state.capabilities.find(c => c.instance === 'sensorHumidity')?.state.value?.currentHumidity;
    const brightness = state.capabilities.find(c => c.instance === 'brightness')?.state.value;

    return { powerState, temperature, brightness, humidity };
  }

  $: status = getDeviceStatus(deviceState);
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
        <div class="power-row">
          <StatusIndicator 
            label="Power" 
            value={status?.powerState ? 'On' : 'Off'}
            icon={status?.powerState ? '⚡' : '💤'}
          />
          <button 
            class="power-button" 
            class:on={status?.powerState}
            on:click={() => onTogglePower(device.device, device.sku, status?.powerState || false)}
          >
            {status?.powerState ? '⏻' : '⭘'}
          </button>
        </div>
        {#if status?.brightness !== undefined && status?.powerState}
          <div class="status-indicator brightness-control">
            <div class="label">
              <span class="icon">✨</span>
              <span>Brightness</span>
            </div>
            <div class="value">
              <span class="brightness-value">{status.brightness}%</span>
              <input 
                type="range"
                min="0"
                max="100"
                value={status.brightness}
                class="brightness-slider"
                on:change={(e) => onChangeCapabilityValue(device.device, device.sku, 'devices.capabilities.brightness', parseInt((e.target as HTMLInputElement).value || '0'), 'brightness')}
              />
            </div>
          </div>
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

  .power-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .power-button {
    background: none;
    border: none;
    color: #ffffff;
    font-size: 1.2rem;
    cursor: pointer;
    padding: 0.5rem;
    border-radius: 50%;
    width: 2.5rem;
    height: 2.5rem;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.2s;
  }

  .power-button:hover {
    background: rgba(255, 255, 255, 0.1);
  }

  .power-button.on {
    color: #4ade80;
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

  .brightness-control {
    padding: 0.5rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
  }

  .brightness-control .value {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .brightness-value {
    min-width: 3rem;
    text-align: right;
  }

  .brightness-slider {
    width: 100px;
    height: 4px;
    -webkit-appearance: none;
    background: rgba(255, 255, 255, 0.1);
    border-radius: 2px;
    outline: none;
  }

  .brightness-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #ffffff;
    cursor: pointer;
  }

  .brightness-slider::-moz-range-thumb {
    width: 12px;
    height: 12px;
    border-radius: 50%;
    background: #ffffff;
    cursor: pointer;
    border: none;
  }
</style>
