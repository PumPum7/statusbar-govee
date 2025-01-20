<script lang="ts">
  import type { Device, DeviceCapabilityInstance, DeviceCapabilityValue, DeviceState, DeviceCapabilityType } from '../types';
  import { colorPresets, numberToRGB, rgbToNumber } from '../types';
  import StatusIndicator from './StatusIndicator.svelte';

  export let device: Device;
  export let deviceState: DeviceState;
  export let onTogglePower: (device: string, sku: string, currentState: boolean) => Promise<void>;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: DeviceCapabilityType, value: DeviceCapabilityValue, instance: DeviceCapabilityInstance) => Promise<void>;

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
      color: getCapabilityValue<number>('colorRgb'),
    };
  }

  $: currentColor = status?.color ? numberToRGB(status.color) : null;

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
            value={""}
            icon={status?.powerState ? '⚡' : '💤'}
          />
          <button 
            class="power-button" 
            class:on={status?.powerState}
            class:off={!status?.powerState}
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
        {#if status?.powerState}
          {#if status?.brightness !== undefined}
            <div class="status-indicator brightness-control">
              <div class="brightness-row">
                <span>Brightness</span>
                <span class="brightness-value">{status.brightness}%</span>
              </div>
              <input 
                type="range"
                min="0"
                max="100"
                value={status.brightness}
                class="brightness-slider"
                style="--value: {status.brightness}%"
                on:input={(e) => {
                  const target = e.target as HTMLInputElement;
                  target.style.setProperty('--value', `${target.value}%`);
                  
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
          {/if}

          {#if deviceState.capabilities.some(c => c.instance === 'colorRgb')}
            <div class="status-indicator color-control">
              <div class="color-row">
                <span>Color</span>
                {#if currentColor}
                  <span class="color-preview" style="background: rgb({currentColor.r}, {currentColor.g}, {currentColor.b})"></span>
                {/if}
              </div>
              <div class="color-presets">
                {#each Object.entries(colorPresets) as [name, color]}
                  <button
                    class="color-preset"
                    aria-label={name}
                    style="background: rgb({color.r}, {color.g}, {color.b})"
                    title={name}
                    on:click={() => {
                      onChangeCapabilityValue(
                        device.device,
                        device.sku,
                        'devices.capabilities.color_setting',
                        rgbToNumber(color),
                        'colorRgb'
                      );
                    }}
                  ></button>
                {/each}
              </div>
              <div class="color-inputs">
                <input
                  type="color"
                  value={`#${currentColor?.r.toString(16).padStart(2, '0')}${currentColor?.g.toString(16).padStart(2, '0')}${currentColor?.b.toString(16).padStart(2, '0')}`}
                  on:input={(e) => {
                    const target = e.target as HTMLInputElement;
                    const hex = target.value.substring(1);
                    const color = {
                      r: parseInt(hex.substring(0, 2), 16),
                      g: parseInt(hex.substring(2, 4), 16),
                      b: parseInt(hex.substring(4, 6), 16)
                    };
                    onChangeCapabilityValue(
                      device.device,
                      device.sku,
                      'devices.capabilities.color_setting',
                      rgbToNumber(color),
                      'colorRgb'
                    );
                  }}
                />
              </div>
            </div>
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

  .power-button.off {
    color: #ff0000;
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
    background: rgba(255, 255, 255, 0.05);
    padding: 12px;
    border-radius: 8px;
  }

  .brightness-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    font-size: 0.9rem;
  }

  .brightness-slider {
    width: 100%;
    height: 4px;
    -webkit-appearance: none;
    appearance: none;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 2px;
    outline: none;
  }

  .brightness-slider::-webkit-slider-thumb {
    -webkit-appearance: none;
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #fff;
    cursor: pointer;
    border: none;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    margin-top: -6px;
  }

  .color-control {
    background: rgba(255, 255, 255, 0.05);
    padding: 12px;
    border-radius: 8px;
    margin-top: 8px;
  }

  .color-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 8px;
    font-size: 0.9rem;
  }

  .color-preview {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: 2px solid rgba(255, 255, 255, 0.1);
  }

  .color-presets {
    display: grid;
    grid-template-columns: repeat(6, 1fr);
    gap: 8px;
    margin-bottom: 8px;
  }

  .color-preset {
    width: 100%;
    aspect-ratio: 1;
    border-radius: 50%;
    border: 2px solid rgba(255, 255, 255, 0.1);
    cursor: pointer;
    padding: 0;
    transition: transform 0.2s;
  }

  .color-preset:hover {
    transform: scale(1.1);
  }

  .color-inputs {
    display: flex;
    gap: 8px;
  }

  .color-inputs input[type="color"] {
    width: 100%;
    height: 40px;
    border: none;
    border-radius: 6px;
    background: rgba(255, 255, 255, 0.1);
    cursor: pointer;
  }

  .color-inputs input[type="color"]::-webkit-color-swatch-wrapper {
    padding: 4px;
  }

  .color-inputs input[type="color"]::-webkit-color-swatch {
    border: none;
    border-radius: 4px;
  }

  .brightness-slider::-moz-range-thumb {
    width: 16px;
    height: 16px;
    border-radius: 50%;
    background: #fff;
    cursor: pointer;
    border: none;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
    margin-top: -6px;
  }

  .brightness-slider::-webkit-slider-runnable-track {
    background: linear-gradient(to right, #fff var(--value, 50%), rgba(255, 255, 255, 0.1) var(--value, 50%));
    border-radius: 2px;
    height: 4px;
  }
</style>
