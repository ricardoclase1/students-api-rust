# 1. Grupo de logs para la Lambda del backend (Tarea 1) - ¡ESTE ES EL QUE IMPORTA!
resource "aws_cloudwatch_log_group" "task1_api_lambda_logs" {
  name              = "/aws/lambda/${aws_lambda_function.task1_api_lambda.function_name}"
  retention_in_days = 7
}

# 2. Grupo de logs para la Lambda que procesa emails
resource "aws_cloudwatch_log_group" "email_processor_lambda_logs" {
  name              = "/aws/lambda/${aws_lambda_function.email_processor_lambda.function_name}"
  retention_in_days = 7
}

# 3. Grupo de logs para la Lambda del BFF
resource "aws_cloudwatch_log_group" "bff_lambda_logs" {
  name              = "/aws/lambda/${aws_lambda_function.bff_lambda.function_name}"
  retention_in_days = 7
}

# 4. Grupo de logs para el API Gateway HTTP del BFF
resource "aws_cloudwatch_log_group" "bff_api_logs" {
  name              = "/aws/apigateway/${aws_apigatewayv2_api.bff_http_api.name}"
  retention_in_days = 7
}