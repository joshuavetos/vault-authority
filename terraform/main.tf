# 1. Enable Required APIs
resource "google_project_service" "services" {
  for_each = toset([
    "sqladmin.googleapis.com",
    "secretmanager.googleapis.com"
  ])
  service            = each.key
  disable_on_destroy = false
}

# 2. Database Password Generation
resource "random_password" "db_password" {
  length  = 24
  special = true
}

# 3. Cloud SQL Instance (PostgreSQL 15)
resource "google_sql_database_instance" "vault_db" {
  name             = "vault-authority-db"
  database_version = "POSTGRES_15"
  region           = var.region
  
  settings {
    tier = "db-f1-micro" # Adjusted for cost; use db-custom-2-7680 for production
    
    ip_configuration {
      ipv4_enabled = true
      # In production, use private_network for INV-3 Boundary Control
    }

    backup_configuration {
      enabled                        = true
      point_in_time_recovery_enabled = true
    }
  }

  deletion_protection = false # Set to true for actual production environments
  depends_on          = [google_project_service.services]
}

# 4. Database User
resource "google_sql_user" "db_user" {
  name     = "vault_admin"
  instance = google_sql_database_instance.vault_db.name
  password = random_password.db_password.result
}

# 5. Secret Manager: Database URL
resource "google_secret_manager_secret" "db_url" {
  secret_id = "vault-db-secrets"
  replication {
    auto {}
  }
}

resource "google_secret_manager_secret_version" "db_url_v1" {
  secret      = google_secret_manager_secret.db_url.id
  secret_data = "postgres://${google_sql_user.db_user.name}:${random_password.db_password.result}@${google_sql_database_instance.vault_db.public_ip_address}:5432/postgres"
}
