use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
group = "farm.w4r.dev",
version = "v1alpha",
kind = "Aviary",
namespaced
)]
pub struct AviarySpec {
    num: i32
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
group = "farm.w4r.dev",
version = "v1alpha",
kind = "Zoo",
namespaced
)]
pub struct ZooSpec {
    num: i32
}

#[derive(CustomResource, Clone, Debug, Deserialize, Serialize, JsonSchema)]
#[kube(
group = "farm.w4r.dev",
version = "v1alpha",
kind = "Aquarium",
namespaced
)]
pub struct AquariumSpec {
    num: i32
}