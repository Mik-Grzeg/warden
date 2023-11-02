allow_k8s_contexts("warden-dev")
k8s_yaml(['k8s/crd.yaml', 'k8s/operator.yaml'])
default_registry('warden-dev-registry:5000')
docker_build('warden-operator', '.', dockerfile='./docker/Dockerfile', build_args={'PROJECT_DIR': 'warden', 'BIN_NAME': 'operator'})
