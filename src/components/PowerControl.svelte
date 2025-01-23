<script lang="ts">
  import type { Device } from '../types';
  import StatusIndicator from './StatusIndicator.svelte';

  export let device: Device;
  export let powerState: boolean;
  export let isPowerLoading: boolean;
  export let onTogglePower: (device: string, sku: string, currentState: boolean) => Promise<void>;

  async function handlePowerToggle() {
    if (isPowerLoading) return;
    await onTogglePower(device.device, device.sku, powerState);
  }
</script>

<div class="power-row">
  <StatusIndicator 
    label="Power" 
    value=""
    icon={powerState ? '⚡' : '💤'}
  />
  <button 
    class="power-button" 
    class:on={powerState}
    class:off={!powerState}
    class:loading={isPowerLoading}
    disabled={isPowerLoading}
    on:click={handlePowerToggle}
  >
    {#if isPowerLoading}
      ⟳
    {:else}
      {powerState ? '⏻' : '⭘'}
    {/if}
  </button>
</div>

<style>
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
</style>