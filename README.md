# Educational project

Kubernetes operator written using kube-rs rust crate. For now it is quite basic version, the only thing it does is reacting to created custom resource `GuardedApplication`. The operator will create pod with mock_app container when the custom resource is created on the cluster.

It requires installation of custom resource definition in the cluster, which is located at k8s/crd.yaml.

## Instructions

### Prerequisites

* [k3d](https://k3d.io/v5.6.0/)
* [tilt](https://tilt.dev/)
* [just](https://github.com/casey/just)

### Starting dev env

Just is used as a command runner, which simplifies starting a dev env.

To start the dev env with live reloading of operator code, run:
```sh
just up
```

What it does:
* creates k3d cluster with local docker registry
* builds mock_app docker image and pushes that to the local docker registry
* deploys operator to the cluster and enables live reloading of operator deployment

### Applying custom resource

In order to see how the operator behaves when custom resource is created, run:
```sh
just apply-cr
```

The operator should react to the creation of custom resource event, and create a pod with mock_app.
