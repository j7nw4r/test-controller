mod schemas;

use std::{sync::Arc, time::Duration};
use futures::StreamExt;
use k8s_openapi::api::apps::v1::Deployment;
use kube::{
    Api, Client, ResourceExt,
    runtime::controller::{Action, Controller}
};
use schemars::schema_for;
use tokio::task::JoinSet;
use log::error;
use tokio::join;
use tokio::sync::oneshot;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("testing failure")]
    TestingFailure,
    #[error("Unknown Error")]
    Unknown
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result<()> {
    let (tx, rx) = oneshot::channel::<()>();
    let mut js = JoinSet::new();
    let _controller_handle = js.spawn(start_controller());
    let _testing_failure_handle = js.spawn(testing_failure());

    while let Some(res) = js.join_next().await {
        println!("saw: {:?}", res);
        match res {
            Ok(task_result) => {
                if let Err(e) = task_result  {

                    return Err(e);
                };
            }
            Err(e) => return Err(Error::Unknown),
        }

    }

    Ok(())
}

async fn testing_failure() -> Result<()> {
    tokio::time::sleep(Duration::from_secs(5)).await;

    Err(Error::TestingFailure)
}

async fn start_controller() -> Result<()> {
    let Ok(client) = Client::try_default().await else {
        return Err(Error::Unknown)
    };
    let deployments = Api::<Deployment>::all(client);

    Controller::new(deployments.clone(), Default::default())
        .run(reconcile, error_policy, Arc::new(()))
        .for_each(|_| futures::future::ready(()))
        .await;

    let buffalo_schema = schema_for!(schemas::BuffaloSpec);
    let Ok(buffalo_json) = serde_json::to_string_pretty(&buffalo_schema) else {
        return Err(Error::Unknown)
    };
    if let Err(_) = std::fs::write("buffalo_spec.json", buffalo_json) {
        return Err(Error::Unknown);
    };

    Ok(())
}

async fn reconcile(obj: Arc<Deployment>, _ctx: Arc<()>) -> Result<Action> {
    println!("reconcile request: {}", obj.name_any());
    Ok(Action::requeue(Duration::from_secs(3600)))
}

fn error_policy(_object: Arc<Deployment>, _err: &Error, _ctx: Arc<()>) -> Action {
    Action::requeue(Duration::from_secs(5))
}