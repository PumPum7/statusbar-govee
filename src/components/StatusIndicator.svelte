<script lang="ts">
  export let label: string;
  export let value: string;
  export let icon: string;
</script>

<div class="status-indicator">
  <div class="label">
    <span class="icon">{icon}</span>
    <span>{label}</span>
  </div>
  <div class="value">{value}</div>
</div>

<style>
  .status-indicator {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background: rgba(255, 255, 255, 0.05);
    border-radius: 8px;
  }

  .label {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: #999;
  }

  .icon {
    font-size: 1.1rem;
  }

  .value {
    font-weight: 500;
    color: #ffffff;
  }
</style>
