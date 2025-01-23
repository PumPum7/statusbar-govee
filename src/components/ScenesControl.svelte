<script lang="ts">
  import type { Device, DeviceCapabilityInstance, DeviceCapabilityType, DeviceCapabilityValue, SceneOption } from '../types';

  export let device: Device;
  export let lightScenes: SceneOption[];
  export let diyScenes: SceneOption[];
  export let showLightScenes: boolean;
  export let showDiyScenes: boolean;
  export let onChangeCapabilityValue: (device: string, sku: string, capabilityType: DeviceCapabilityType, value: DeviceCapabilityValue, instance: DeviceCapabilityInstance) => Promise<void>;
</script>

{#if lightScenes.length > 0}
  <div class="scene-control">
    <button 
      class="scene-toggle"
      on:click={() => showLightScenes = !showLightScenes}
      aria-expanded={showLightScenes}
    >
      <span>Light Scenes {showLightScenes ? '▼' : '▶'}</span>
    </button>
    {#if showLightScenes}
      <div class="scene-list">
        {#each lightScenes as scene}
          <button
            class="scene-button"
            on:click={() => {
              onChangeCapabilityValue(
                device.device,
                device.sku,
                'devices.capabilities.dynamic_scene',
                scene.value.id,
                'lightScene'
              );
            }}
          >
            {scene.name}
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/if}

{#if diyScenes.length > 0}
  <div class="scene-control">
    <button 
      class="scene-toggle"
      on:click={() => showDiyScenes = !showDiyScenes}
      aria-expanded={showDiyScenes}
    >
      <span>DIY Scenes {showDiyScenes ? '▼' : '▶'}</span>
    </button>
    {#if showDiyScenes}
      <div class="scene-list">
        {#each diyScenes as scene}
          <button
            class="scene-button"
            on:click={() => {
              onChangeCapabilityValue(
                device.device,
                device.sku,
                'devices.capabilities.dynamic_scene',
                scene.value.id,
                'diyScene'
              );
            }}
          >
            {scene.name}
          </button>
        {/each}
      </div>
    {/if}
  </div>
{/if}

<style>
  .scene-control {
    margin: 1rem 0;
  }

  .scene-toggle {
    width: 100%;
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    color: #fff;
    padding: 0.5rem;
    font-size: 0.9rem;
    cursor: pointer;
    display: flex;
    justify-content: space-between;
    align-items: center;
    transition: background-color 0.2s;
  }

  .scene-toggle:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .scene-list {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 0.5rem;
    margin-top: 0.5rem;
  }

  .scene-button {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    border-radius: 4px;
    color: #fff;
    padding: 0.5rem;
    font-size: 0.8rem;
    cursor: pointer;
    transition: all 0.2s;
  }

  .scene-button:hover {
    background: rgba(255, 255, 255, 0.2);
  }
</style>