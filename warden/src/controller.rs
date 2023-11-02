use futures::StreamExt;
use k8s_openapi::api::core::v1::{Container, ContainerPort, HTTPGetAction, Pod, PodSpec, Probe};
use k8s_openapi::apimachinery::pkg::util::intstr::IntOrString;
use kube::api::PostParams;
use kube::{
    api::ListParams,
    core::ObjectMeta,
    runtime::{controller::Action, watcher::Config, Controller},
    Api, Client, CustomResource, Resource,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[cfg_attr(test, derive(Default))]
#[kube(
    kind = "GuardedApplication",
    group = "kube.rs",
    version = "v1",
    namespaced
)]
#[kube(status = "GuardedApplicationStatus", shortname = "app")]
pub struct GuardedApplicationSpec {
    pub title: String,
    pub replicas: usize,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug, JsonSchema)]
pub struct GuardedApplicationStatus {
    replicas_up_to_date: bool,
}

// impl GuardedApplication {
//     fn is_up_to_date(&self) -> bool {
//         self.status
//             .as_ref()
//             .map(|s| s.replicas_up_to_date)
//             .unwrap_or(false)
//     }
// }

#[derive(Clone)]
pub struct Context {
    /// Kubernetes client
    pub client: Client,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error("K8s API error: {0}")]
    KubeError(#[source] kube::Error),
    #[error("Namespace is not defined")]
    MissingNamespace,
}

fn error_policy(
    _guarded_app: Arc<GuardedApplication>,
    error: &Error,
    _ctx: Arc<Context>,
) -> Action {
    tracing::warn!("Reconcilation failed: {:?}", error);
    Action::requeue(Duration::from_secs(5 * 60))
}

async fn reconcile(
    guarded_app: Arc<GuardedApplication>,
    ctx: Arc<Context>,
) -> Result<Action, Error> {
    let client = &ctx.client;

    let oref = guarded_app.controller_owner_ref(&()).unwrap();
    let pod_template = Pod {
        metadata: ObjectMeta {
            name: guarded_app.metadata.name.clone(),
            owner_references: Some(vec![oref]),
            ..ObjectMeta::default()
        },
        spec: Some(PodSpec {
            containers: vec![Container {
                image: Some("warden-dev-registry:5000/mock_app:latest".into()),
                name: guarded_app
                    .metadata
                    .name
                    .clone()
                    .unwrap_or_else(|| "app".into()),
                ports: Some(vec![ContainerPort {
                    container_port: 8000,
                    ..ContainerPort::default()
                }]),
                liveness_probe: Some(Probe {
                    http_get: Some(HTTPGetAction {
                        path: Some("/health".into()),
                        port: IntOrString::Int(8000),
                        ..HTTPGetAction::default()
                    }),
                    ..Probe::default()
                }),
                ..Container::default()
            }],
            ..PodSpec::default()
        }),
        ..Pod::default()
    };

    let pod_api = Api::<Pod>::namespaced(
        client.clone(),
        guarded_app
            .metadata
            .namespace
            .as_ref()
            .ok_or(Error::MissingNamespace)?,
    );

    pod_api
        .create(
            &PostParams {
                dry_run: false,
                field_manager: Some("warden-operator".into()),
            },
            &pod_template,
        )
        .await
        .map_err(Error::KubeError)?;

    Ok(Action::requeue(Duration::from_secs(5 * 60)))
}

pub async fn run() -> Result<(), anyhow::Error> {
    let client = Client::try_default()
        .await
        .expect("Should be able to create k8s client");
    let guarded_apps = Api::<GuardedApplication>::all(client.clone());

    if let Err(err) = guarded_apps.list(&ListParams::default().limit(1)).await {
        tracing::error!("CRD not queryable: {err:?}");
        std::process::exit(1)
    }

    Controller::new(guarded_apps, Config::default().any_semantic())
        .shutdown_on_signal()
        .run(reconcile, error_policy, Arc::new(Context { client }))
        .for_each(|res| async move {
            match res {
                Ok(o) => tracing::info!("reconciled {:?}", o),
                Err(e) => tracing::warn!("reconcilation failed: {}", e),
            }
        })
        .await;
    Ok(())
}
