<script lang="ts">
  import type { Device, DeviceCapabilityInstance, DeviceCapabilityValue, DeviceState } from '../types';
  import StatusIndicator from './StatusIndicator.svelte';

  export let device: Device;
  export let deviceState: DeviceState;
  export let onTogglePower: (device: string, sku: string, currentState: boolean) => Promise<void>;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: string, value: number, instance: string) => Promise<void>;

  let isPowerLoading = false;
  let error: string | null = null;

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
    };
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
            class:loading={isPowerLoading}
            disabled={isPowerLoading}
            on:click={handlePowerToggle}
          >
            {#if isPowerLoading}
              ⟳
            {:else}
              {status?.powerState ? '⏻' : '⭘'}
            {/if}
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
                on:input={(e) => {
                  const target = e.target as HTMLInputElement;
                  clearTimeout(target.dataset.timeout as any);
                  target.dataset.timeout = setTimeout(() => {
                    onChangeCapabilityValue(
                      device.device, 
                      device.sku, 
                      'devices.capabilities.brightness', 
                      parseInt(target.value || '0'), 
                      'brightness'
                    );
                  }, 300) as any;
                }}
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

  .power-button.loading {
    animation: spin 1s linear infinite;
  }

  @keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
  }

  .power-button:disabled {
    opacity: 0.7;
    cursor: not-allowed;
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
    appearance: none;
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
