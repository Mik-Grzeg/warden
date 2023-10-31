allow_k8s_contexts("warden-dev")
k8s_yaml(['k8s/crd.yaml', 'k8s/operator_deployment.yaml'])
docker_build('warden', '.', dockerfile='./docker/Dockerfile', build_args={'DEBUG': 'true'})
