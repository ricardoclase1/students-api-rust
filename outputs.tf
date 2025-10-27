# --- ARCHIVO outputs.tf DESACTIVADO TEMPORALMENTE PARA LA ENTREGA ---
# La mayoría de estos outputs se refieren a recursos que hemos comentado
# en otros archivos para poder desplegar el backend principal.

# output "task1_api_invoke_url" {
#   description = "La URL de invocación para el API Gateway de la Tarea 1."
#   value       = aws_api_gateway_stage.api_stage.invoke_url
# }
# 
# output "bff_api_key_value" {
#   description = "El valor de la clave de API para que el BFF acceda a la Tarea 1."
#   value       = aws_api_gateway_api_key.bff_api_key.value
#   sensitive   = true
# }
# 
# output "sns_topic_arn" {
#   description = "El ARN del topic de SNS para notificaciones."
#   value       = aws_sns_topic.email_notifications.arn
# }
# 
# output "sqs_queue_arn" {
#   description = "El ARN de la cola SQS de emails."
#   value       = aws_sqs_queue.email_queue.arn
# }
# 
# output "sqs_queue_url" {
#   description = "La URL de la cola SQS de emails."
#   value       = aws_sqs_queue.email_queue.id
# }
# 
# output "ecr_repository_url" {
#   description = "La URL del repositorio ECR para subir la imagen de la Tarea 1."
#   value       = aws_ecr_repository.task1_api_repo.repository_url
# }
# 
# output "bff_api_invoke_url" {
#   description = "La URL de invocación para el API Gateway HTTP del BFF."
#   value       = aws_apigatewayv2_stage.bff_stage.invoke_url
# }
# 
# output "email_dlq_arn" {
#   description = "El ARN de la Dead Letter Queue para emails fallidos."
#   value       = aws_sqs_queue.email_dlq.arn
# }
# 
# output "email_dlq_url" {
#   description = "La URL de la Dead Letter Queue para emails fallidos."
#   value       = aws_sqs_queue.email_dlq.id
# }