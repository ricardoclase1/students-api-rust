variable "aws_region" {
  description = "La región de AWS donde se desplegarán los recursos."
  type        = string
  default     = "us-east-1" # Puedes cambiarla por tu región preferida
}

variable "jwt_secret" {
  description = "Secreto para firmar y verificar JWTs. Debe ser una cadena segura."
  type        = string
  sensitive   = true # Para que Terraform no muestre el valor en los logs
}