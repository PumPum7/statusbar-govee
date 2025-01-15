export type Device = {
  device: string;
  deviceName?: string;
  sku: string;
  type: string;
};

export type DeviceState = {
  capabilities: Array<{
    capability_type: string;
    instance: string;
    state: { value: any };
  }>;
};
