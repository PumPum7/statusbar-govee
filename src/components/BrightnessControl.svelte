<script lang="ts">
  import type { Device, DeviceCapabilityInstance, DeviceCapabilityType, DeviceCapabilityValue } from '../types';

  export let device: Device;
  export let brightness: number;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: DeviceCapabilityType, value: DeviceCapabilityValue, instance: DeviceCapabilityInstance) => Promise<void>;
</script>

<div class="status-indicator brightness-control">
  <div class="brightness-row">
    <span>Brightness</span>
    <span class="brightness-value">{brightness}%</span>
  </div>
  <input 
    type="range"
    min="0"
    max="100"
    value={brightness}
    class="brightness-slider"
    style="--value: {brightness}%"
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

<style>
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