# Grant the Kubernetes Service Account access to secrets
resource "google_secret_manager_secret_iam_member" "vault_accessor" {
  project   = var.project_id
  secret_id = google_secret_manager_secret.db_url.secret_id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${var.project_id}.svc.id.goog[default/vault-authority]"
}
