<script lang="ts">
  import DeviceCard from './DeviceCard.svelte';
  import type { Device, DeviceState } from '../types';

  export let devices: Device[] = [];
  export let deviceStates: Record<string, DeviceState> = {};
  export let onRefresh: () => Promise<void>;
  export let onTogglePower: (device: string, sku: string, currentState: boolean) => Promise<void>;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: string, value: number, instance: string) => Promise<void>;
</script>

<div class="device-list-container">
  <div class="header">
    <h1>Govee Devices</h1>
    <button class="refresh-button" on:click={onRefresh}>
      ↻ Refresh
    </button>
  </div>

  {#if devices.length > 0}
    <div class="device-list">
      {#each devices as device}
        {#if deviceStates[device.device] && deviceStates[device.device].capabilities.find(c => c.instance === 'online')?.state.value === true}
          <DeviceCard 
            {device}
            deviceState={deviceStates[device.device]}
            onTogglePower={onTogglePower}
            onChangeCapabilityValue={onChangeCapabilityValue}
          />
        {/if}
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <p>No devices found</p>
      <p class="subtitle">Click refresh to fetch devices</p>
    </div>
  {/if}
</div>

<style>
  .device-list-container {
    padding: 1rem;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1.5rem;
  }

  h1 {
    font-size: 1.5rem;
    margin: 0;
    color: #ffffff;
  }

  .refresh-button {
    background: #4a4a4a;
    border: none;
    color: #ffffff;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    font-size: 0.9rem;
    transition: background-color 0.2s;
  }

  .refresh-button:hover {
    background: #5a5a5a;
  }

  .device-list {
    display: grid;
    gap: 1rem;
  }

  .empty-state {
    text-align: center;
    padding: 2rem;
    color: #999;
  }

  .empty-state p {
    margin: 0;
  }

  .subtitle {
    font-size: 0.9rem;
    opacity: 0.7;
  }
</style>
