export type Device = {
  device: string;
  deviceName?: string;
  sku: string;
  type: string;
};

export type DeviceCapabilityType =
  | "devices.capabilities.on_off"
  | "devices.capabilities.brightness"
  | "devices.capabilities.color_setting"
  | "devices.capabilities.online"
  | "devices.capabilities.sensorTemperature"
  | "devices.capabilities.sensorHumidity"
  | "devices.capabilities.diy_color_setting"
  | "devices.capabilities.dynamic_scene";

export type DeviceCapabilityInstance =
  | "powerSwitch"
  | "brightness"
  | "color"
  | "online"
  | "sensorTemperature"
  | "sensorHumidity"
  | "colorRgb"
  | "diyScene"
  | "lightScene";
  
export type RGBColor = {
  r: number;
  g: number;
  b: number;
};

export type DeviceCapabilityValue =
  | boolean
  | number
  | { currentHumidity: number }
  | number;

// Convert RGB color object to a single number
export function rgbToNumber(color: RGBColor): number {
  return (
    ((color.r & 0xff) << 16) | ((color.g & 0xff) << 8) | ((color.b & 0xff) << 0)
  );
}

// Convert a number to RGB color object
export function numberToRGB(num: number): RGBColor {
  return {
    r: (num >> 16) & 255,
    g: (num >> 8) & 255,
    b: num & 255,
  };
}

// Common color presets
export const colorPresets: Record<string, RGBColor> = {
  "Warm White": { r: 255, g: 244, b: 229 },
  "Cool White": { r: 255, g: 255, b: 255 },
  Red: { r: 255, g: 0, b: 0 },
  Green: { r: 0, g: 255, b: 0 },
  Blue: { r: 0, g: 0, b: 255 },
  Purple: { r: 128, g: 0, b: 128 },
};

export type DeviceState = {
  capabilities: Array<{
    capability_type: DeviceCapabilityType;
    instance: DeviceCapabilityInstance;
    state: { value: DeviceCapabilityValue };
  }>;
};

export type SceneOption = {
  name: String;
  value: {
    id: number;
    paramId: number,
  }
};