use axum::{http::StatusCode, Json};
use cuid::cuid2;
use serde::{Deserialize, Serialize};

use crate::bcrypt::Bcrypt;

use super::{device::Device, password_generator::PasswordGenerator};

#[derive(Deserialize)]
pub struct CreateDeviceRequest {
    name: String,
}

#[derive(Serialize)]
pub struct CreateDeviceResponse {
    id: String,
    name: String,
    password: String,
}

impl From<Device> for CreateDeviceResponse {
    fn from(device: Device) -> Self {
        CreateDeviceResponse {
            id: device.id,
            name: device.name,
            password: device.password,
        }
    }
}

pub async fn create_device(
    Json(payload): Json<CreateDeviceRequest>,
) -> (StatusCode, Json<CreateDeviceResponse>) {
    let unhashed = PasswordGenerator::generate_default();
    let hashed = Bcrypt::hash(&unhashed);

    let mut device = Device {
        id: cuid2(),
        name: payload.name,
        password: hashed,
    };

    // TODO: save db

    // The device will store the unhashed password
    device.password = unhashed;

    (StatusCode::CREATED, Json(device.into()))
}
