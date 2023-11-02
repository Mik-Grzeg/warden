cluster_name := "warden-dev"
registry := "warden-dev-registry"
registry_port := "5432"

start-dev:
  @echo "Starting {{cluster_name}} cluster..."
  @k3d cluster list | grep {{cluster_name}} > /dev/null || k3d cluster create {{cluster_name}}  --registry-create {{registry}}:{{registry_port}} 2> /dev/null

stop-dev:
  @k3d cluster delete {{cluster_name}} 1>/dev/null

generate-crd:
  cargo run --manifest-path warden/Cargo.toml --bin crdgen > kubectl apply -f -

apply-cr:
  kubectl apply -f k8s/mock_app_cr.yaml

up:
  just start-dev
  just build_mock
  @tilt up --stream

build_mock:
  #!/usr/bin/env bash
  set -euxo pipefail
  mock_app_image="mock_app:latest"
  docker build --build-arg="PROJECT_DIR=mock_app" --build-arg="BIN_NAME=mock_app" -t {{registry}}.localhost:{{registry_port}}/$mock_app_image -f docker/Dockerfile .
  docker push {{registry}}.localhost:{{registry_port}}/$mock_app_image

