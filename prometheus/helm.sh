#!/bin/bash
helm install prometheus prometheus-community/kube-prometheus-stack --namespace=prometheus --create-namespace --wait -f /values.yaml


# metrics server
helm repo add metrics-server https://kubernetes-sigs.github.io/metrics-server/

