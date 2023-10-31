cluster_name := "warden-dev"

start-dev:
  @echo "Starting {{cluster_name}} cluster..."
  @k3d cluster list | grep {{cluster_name}} > /dev/null || k3d cluster create {{cluster_name}}  --registry-create {{cluster_name}}-registry 2> /dev/null

stop-dev:
  @k3d cluster delete {{cluster_name}} 1>/dev/null

generate-crd:
  cargo run --bin crdgen > k8s/crd.yaml

up:
  just start-dev
  @tilt up --stream

enable-telemetry:
  helm repo add open-telemetry https://open-telemetry.github.io/opentelemetry-helm-charts
  helm install my-opentelemetry-operator open-telemetry/opentelemetry-operator \
    --set admissionWebhooks.certManager.enabled=false \
    --set admissionWebhooks.certManager.autoGenerateCert=true



