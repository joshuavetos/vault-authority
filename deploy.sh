#!/bin/bash
# Vault Authority Production Deployment Orchestrator
# Enforces monotonic state transitions from build to cluster.

set -e # Exit immediately on error

# 1. Configuration & Auto-Discovery
PROJECT_ID=$(gcloud config get-value project)
REGION="us-central1"
IMAGE_TAG="v1.0.0"
IMAGE_URL="gcr.io/${PROJECT_ID}/vault-authority:${IMAGE_TAG}"
SA_NAME="vault-authority-sa"
SA_EMAIL="${SA_NAME}@${PROJECT_ID}.iam.gserviceaccount.com"

echo "🚀 Starting Deployment for Project: ${PROJECT_ID}"

# 2. Sequential Build & Push (INV-1)
echo "📦 Building and Pushing Container Image..."
gcloud builds submit --tag "${IMAGE_URL}" .

# 3. Identity Binding & Permissions (INV-3)
echo "🔑 Updating Workload Identity Bindings..."
gcloud secrets add-iam-policy-binding vault-db-url \
    --role="roles/secretmanager.secretAccessor" \
    --member="serviceAccount:${PROJECT_ID}.svc.id.goog[default/${SA_NAME}]" \
    --quiet

kubectl annotate serviceaccount "${SA_NAME}" \
    iam.gke.io/gcp-service-account="${SA_EMAIL}" \
    --overwrite

# 4. Atomic Deployment (INV-2)
echo "☸️ Applying Kubernetes Manifests..."

# Update the deployment file with the real Project ID dynamically
sed -i "s|gcr.io/your-project/vault-authority:v1.0.0|${IMAGE_URL}|g" k8s/deployment-csi.yaml

kubectl apply -f k8s/secret-provider.yaml
kubectl apply -f k8s/deployment-csi.yaml

echo "✅ Deployment Complete. Run 'kubectl get pods' to verify status."
