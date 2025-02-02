use std::sync::Once;

use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use tauri_nspanel::ManagerExt;
use tauri_plugin_store::StoreExt;

use crate::fns::{
    setup_menubar_panel_listeners, swizzle_to_menubar_panel, update_menubar_appearance,
};

static INIT: Once = Once::new();
const SETTINGS_FILE: &str = "settings.json";

#[derive(Debug, Serialize, Deserialize)]
struct GoveeResponse {
    code: i32,
    message: String,
    data: Vec<GoveeDevice>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoveeDevice {
    sku: String,
    device: String,
    #[serde(rename = "type")]
    device_type: String,
    #[serde(default, rename = "deviceName")]
    device_name: Option<String>,
    capabilities: Vec<GoveeCapability>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GoveeCapability {
    #[serde(rename = "type")]
    capability_type: String,
    instance: String,
    #[serde(default)]
    parameters: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceStateRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: DeviceStatePayload,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceStatePayload {
    device: String,
    sku: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DeviceStateResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    code: i32,
    msg: String,
    payload: DeviceState,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceState {
    capabilities: Vec<CapabilityState>,
    device: String,
    sku: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct CapabilityState {
    #[serde(rename = "type")]
    capability_type: String,
    instance: String,
    state: StateValue,
}

#[derive(Debug, Serialize, Deserialize)]
struct StateValue {
    value: serde_json::Value,
}

#[derive(Debug, Serialize)]
struct DeviceControlRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: DeviceControlPayload,
}

#[derive(Debug, Serialize)]
struct DeviceControlPayload {
    device: String,
    sku: String,
    capability: CapabilityControl,
}

#[derive(Debug, Serialize)]
struct CapabilityControl {
    #[serde(rename = "type")]
    capability_type: String,
    instance: String,
    value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicSceneRequest {
    #[serde(rename = "requestId")]
    request_id: String,
    payload: DeviceStatePayload,
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicSceneResponse {
    #[serde(rename = "requestId")]
    request_id: String,
    code: i32,
    msg: String,
    payload: DynamicScenePayload,
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicScenePayload {
    sku: String,
    device: String,
    capabilities: Vec<DynamicSceneCapability>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicSceneCapability {
    #[serde(rename = "type")]
    capability_type: String,
    instance: String,
    parameters: DynamicSceneParameters,
}

#[derive(Debug, Serialize, Deserialize)]
struct DynamicSceneParameters {
    #[serde(rename = "dataType")]
    data_type: String,
    options: Vec<SceneOption>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SceneOption {
    name: String,
    value: serde_json::Value, // Can be integer or object with paramId and id
}

fn generate_request_id() -> String {
    Uuid::new_v4().to_string()
}

#[tauri::command]
pub fn init(app_handle: tauri::AppHandle) {
    INIT.call_once(|| {
        swizzle_to_menubar_panel(&app_handle);

        update_menubar_appearance(&app_handle);

        setup_menubar_panel_listeners(&app_handle);
    });
}

#[tauri::command]
pub fn show_menubar_panel(app_handle: tauri::AppHandle) {
    let panel = app_handle.get_webview_panel("main").unwrap();

    panel.show();
}

#[tauri::command]
pub async fn get_devices(app: tauri::AppHandle) -> Result<Vec<GoveeDevice>, String> {
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let api_key: Option<String> = store
        .get("api_key")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let api_key = api_key.ok_or("API key not set. Please set your Govee API key first.")?;

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Govee-API-Key",
        HeaderValue::from_str(&api_key).map_err(|e| format!("Invalid API key format: {}", e))?,
    );

    let response = client
        .get("https://openapi.api.govee.com/router/api/v1/user/devices")
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch devices: {}", e))?;

    let mut govee_response: GoveeResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Clean up the type field by removing the "devices.types." prefix
    for device in &mut govee_response.data {
        if let Some(stripped_type) = device.device_type.strip_prefix("devices.types.") {
            device.device_type = stripped_type.to_string();
        }
    }

    if govee_response.code != 200 {
        return Err(format!("API error: {}", govee_response.message));
    }

    Ok(govee_response.data)
}

#[tauri::command]
pub async fn get_device_state(
    app: tauri::AppHandle,
    device: String,
    sku: String,
) -> Result<DeviceState, String> {
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let api_key: Option<String> = store
        .get("api_key")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let api_key = api_key.ok_or("API key not set. Please set your Govee API key first.")?;

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Govee-API-Key",
        HeaderValue::from_str(&api_key).map_err(|e| format!("Invalid API key format: {}", e))?,
    );
    let request_body = DeviceStateRequest {
        request_id: generate_request_id(),
        payload: DeviceStatePayload { device, sku },
    };

    let response = client
        .post("https://openapi.api.govee.com/router/api/v1/device/state")
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch device state: {}", e))?;

    let mut state_response: DeviceStateResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Convert temperature from Fahrenheit to Celsius if sensorTemperature capability exists
    if let Some(temp_cap) = state_response
        .payload
        .capabilities
        .iter_mut()
        .find(|c| c.instance == "sensorTemperature")
    {
        if let Some(temp_f) = temp_cap.state.value.as_f64() {
            temp_cap.state.value = serde_json::Value::from(((temp_f - 32.0) * 5.0 / 9.0).round());
        }
    }

    if state_response.code != 200 {
        return Err(format!("API error: {}", state_response.msg));
    }

    Ok(state_response.payload)
}

#[tauri::command]
pub async fn change_capability_value(
    app: tauri::AppHandle,
    device: String,
    sku: String,
    capability_type: String,
    instance: String,
    value: serde_json::Value,
) -> Result<(), String> {
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let api_key: Option<String> = store
        .get("api_key")
        .and_then(|v| v.as_str().map(|s| s.to_string()));

    let api_key = api_key.ok_or("API key not set. Please set your Govee API key first.")?;

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Govee-API-Key",
        HeaderValue::from_str(&api_key).map_err(|e| format!("Invalid API key format: {}", e))?,
    );

    let request = DeviceControlRequest {
        request_id: generate_request_id(),
        payload: DeviceControlPayload {
            device,
            sku,
            capability: CapabilityControl {
                capability_type,
                instance,
                value,
            },
        },
    };

    let response = client
        .post("https://openapi.api.govee.com/router/api/v1/device/control")
        .headers(headers)
        .json(&request)
        .send()
        .await
        .map_err(|e| format!("Failed to send control command: {}", e))?;

    let status = response.status();
    if !status.is_success() {
        let error_text = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("API error ({}): {}", status, error_text));
    }

    Ok(())
}

#[tauri::command]
pub fn get_api_key(app: tauri::AppHandle) -> Option<String> {
    let store = app.store(SETTINGS_FILE).ok()?;
    store
        .get("api_key")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
}

#[tauri::command]
pub async fn set_api_key(app: tauri::AppHandle, api_key: String) -> Result<(), String> {
    // Test if the API key is valid
    if api_key.is_empty() {
        return Err("API key cannot be empty".to_string());
    }

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Govee-API-Key",
        HeaderValue::from_str(&api_key).map_err(|e| format!("Invalid API key format: {}", e))?,
    );
    let response = client
        .get("https://openapi.api.govee.com/router/api/v1/user/devices")
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch devices: {}", e))?;

    if response.status().is_success() {
        let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
        store.set("api_key", serde_json::Value::String(api_key));
        store
            .save()
            .map_err(|_| "Failed to save settings".to_string())?;
        Ok(())
    } else {
        Err("Invalid API key".to_string())
    }
}

#[tauri::command]
pub async fn get_light_scenes(
    app: tauri::AppHandle,
    device: String,
    sku: String,
) -> Result<Vec<SceneOption>, String> {
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let api_key: Option<String> = store
        .get("api_key")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let api_key = api_key.ok_or("API key not set. Please set your Govee API key first.")?;

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Govee-API-Key",
        HeaderValue::from_str(&api_key).map_err(|e| format!("Invalid API key format: {}", e))?,
    );

    let request_body = DynamicSceneRequest {
        request_id: generate_request_id(),
        payload: DeviceStatePayload { device, sku },
    };

    let response = client
        .post("https://openapi.api.govee.com/router/api/v1/device/scenes")
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch light scenes: {}", e))?;

    let scene_response: DynamicSceneResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if scene_response.code != 200 {
        return Err(format!("API error: {}", scene_response.msg));
    }

    // Find the lightScene capability and return its options
    let light_scenes = scene_response
        .payload
        .capabilities
        .iter()
        .find(|cap| cap.instance == "lightScene")
        .ok_or("No light scenes found")?;

    Ok(light_scenes.parameters.options.clone())
}

#[tauri::command]
pub async fn get_diy_scenes(
    app: tauri::AppHandle,
    device: String,
    sku: String,
) -> Result<Vec<SceneOption>, String> {
    let store = app.store(SETTINGS_FILE).map_err(|e| e.to_string())?;
    let api_key: Option<String> = store
        .get("api_key")
        .and_then(|v| v.as_str().map(|s| s.to_string()));
    let api_key = api_key.ok_or("API key not set. Please set your Govee API key first.")?;

    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(
        "Govee-API-Key",
        HeaderValue::from_str(&api_key).map_err(|e| format!("Invalid API key format: {}", e))?,
    );

    let request_body = DynamicSceneRequest {
        request_id: generate_request_id(),
        payload: DeviceStatePayload { device, sku },
    };

    let response = client
        .post("https://openapi.api.govee.com/router/api/v1/device/diy-scenes")
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .map_err(|e| format!("Failed to fetch DIY scenes: {}", e))?;

    let scene_response: DynamicSceneResponse = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    if scene_response.code != 200 {
        return Err(format!("API error: {}", scene_response.msg));
    }

    // Find the diyScene capability and return its options
    let diy_scenes = scene_response
        .payload
        .capabilities
        .iter()
        .find(|cap| cap.instance == "diyScene")
        .ok_or("No DIY scenes found")?;

    Ok(diy_scenes.parameters.options.clone())
}
