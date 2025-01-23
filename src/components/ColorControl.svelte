<script lang="ts">
  import type { Device, DeviceCapabilityInstance, DeviceCapabilityType, DeviceCapabilityValue, RGBColor } from '../types';
  import { colorPresets, numberToRGB, rgbToNumber } from '../types';

  export let device: Device;
  export let currentColor: RGBColor | null;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: DeviceCapabilityType, value: DeviceCapabilityValue, instance: DeviceCapabilityInstance) => Promise<void>;
</script>

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

<style>
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
</style>