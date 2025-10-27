# 1. Lambda para el backend principal (Tarea 1)
resource "aws_lambda_function" "task1_api_lambda" {
  function_name = "${var.project_name}-task1-api"
  role          = aws_iam_role.task1_lambda_role.arn
  
  package_type  = "Zip"
  handler       = "bootstrap"
  runtime       = "provided.al2023"
  filename      = "deployment.zip"
  source_code_hash = filebase64sha256("deployment.zip")

  # --- CONFIGURACIÓN CRÍTICA CORREGIDA ---
  architectures = ["x86_64"]
  timeout       = 30
  memory_size   = 512
  # ------------------------------------

  environment {
    variables = {
      JWT_SECRET   = var.jwt_secret
    }
  }
}

# 2. Lambda para procesar los correos desde la cola SQS (Python)
resource "aws_lambda_function" "email_processor_lambda" {
  function_name = "${var.project_name}-email-processor"
  role          = aws_iam_role.email_processor_lambda_role.arn
  handler       = "email_processor.lambda_handler"
  runtime       = "python3.9"
  architectures = ["x86_64"]

  filename         = "lambdas/email-processor/email_processor.zip"
  source_code_hash = filebase64sha256("lambdas/email-processor/email_processor.zip")

  timeout = 15

  environment {
    variables = {
      LOG_LEVEL = "INFO"
    }
  }
}

# 3. Mapeo de eventos: Conecta la cola SQS con la Lambda de emails
resource "aws_lambda_event_source_mapping" "email_queue_trigger" {
  event_source_arn = aws_sqs_queue.email_queue.arn
  function_name    = aws_lambda_function.email_processor_lambda.arn
  batch_size       = 5

  depends_on = [aws_iam_role_policy_attachment.email_processor_sqs]
}

# 4. Lambda para el BFF (Backend for Frontend)
resource "aws_lambda_function" "bff_lambda" {
  function_name = "${var.project_name}-bff"
  role          = aws_iam_role.bff_lambda_role.arn

  package_type  = "Zip"
  handler       = "bootstrap"
  runtime       = "provided.al2023"
  filename      = "bff-deployment.zip"
  source_code_hash = filebase64sha256("bff-deployment.zip")

  timeout = 30

  environment {
    variables = {
      API_GATEWAY_URL = aws_api_gateway_stage.api_stage.invoke_url
      API_KEY         = aws_api_gateway_api_key.bff_api_key.value
      SNS_TOPIC_ARN   = aws_sns_topic.email_notifications.arn
    }
  }
}