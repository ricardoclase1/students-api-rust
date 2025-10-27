variable "aws_region" {
  description = "La región de AWS donde se desplegarán los recursos."
  type        = string
  default     = "us-east-1"
}

variable "jwt_secret" {
  description = "Secreto para firmar y verificar JWTs. Debe ser una cadena segura."
  type        = string
  sensitive   = true
}

variable "project_name" {
  description = "Nombre base para los recursos del proyecto."
  type        = string
  default     = "students-api"
}