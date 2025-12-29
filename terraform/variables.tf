variable "project_id" {
  description = "The GCP Project ID"
  type        = string
}

variable "region" {
  description = "Region for database and secrets"
  type        = string
  default     = "us-central1"
}
