export type Device = {
  device: string;
  deviceName?: string;
  sku: string;
  type: string;
};

export type DeviceCapabilityType =
  | "devices.capabilities.on_off"
  | "devices.capabilities.brightness"
  | "devices.capabilities.color"
  | "devices.capabilities.online"
  | "devices.capabilities.sensorTemperature"
  | "devices.capabilities.sensorHumidity";

export type DeviceCapabilityInstance =
  | "powerSwitch"
  | "brightness"
  | "color"
  | "online"
  | "sensorTemperature"
  | "sensorHumidity";

export type DeviceCapabilityValue =
  | boolean
  | number
  | { currentHumidity: number }
  | { r: number; g: number; b: number };

export type DeviceState = {
  capabilities: Array<{
    capability_type: DeviceCapabilityType;
    instance: DeviceCapabilityInstance;
    state: { value: DeviceCapabilityValue };
  }>;
};
