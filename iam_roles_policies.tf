# 1. Rol para la Lambda del backend principal (Tarea 1)
resource "aws_iam_role" "task1_lambda_role" {
  name = "students-api-task1-lambda-role" # Ajustamos al nombre que tenías para consistencia

  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{
      Action = "sts:AssumeRole",
      Effect = "Allow",
      Principal = {
        Service = "lambda.amazonaws.com"
      }
    }]
  })
}

resource "aws_iam_role_policy_attachment" "task1_lambda_logs" {
  role       = aws_iam_role.task1_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole" # Esto ya estaba correcto
}

# 2. Rol para la Lambda que procesa emails (email-processor)
resource "aws_iam_role" "email_processor_lambda_role" {
  name = "${var.project_name}-email-processor-lambda-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{
      Action = "sts:AssumeRole",
      Effect = "Allow",
      Principal = {
        Service = "lambda.amazonaws.com"
      }
    }]
  })
}

# Política para que la Lambda de emails pueda leer y borrar mensajes de SQS
resource "aws_iam_policy" "email_processor_sqs_policy" {
  name        = "${var.project_name}-email-processor-sqs-policy"
  description = "Permite a la Lambda leer y borrar mensajes de la cola SQS de emails."

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Action = [
          "sqs:ReceiveMessage",
          "sqs:DeleteMessage",
          "sqs:GetQueueAttributes"
        ],
        Resource = aws_sqs_queue.email_queue.arn
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "email_processor_sqs" {
  role       = aws_iam_role.email_processor_lambda_role.name
  policy_arn = aws_iam_policy.email_processor_sqs_policy.arn
}

resource "aws_iam_role_policy_attachment" "email_processor_logs" {
  role       = aws_iam_role.email_processor_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

# 3. Repositorio ECR para la imagen de la Tarea 1
resource "aws_ecr_repository" "task1_api_repo" {
  name                 = "${var.project_name}-task1-repo"
  image_tag_mutability = "MUTABLE"

  image_scanning_configuration {
    scan_on_push = true
  }
}

# 4. Política para que el BFF pueda publicar en el Topic SNS
# Esta política se puede adjuntar a un rol de IAM (si el BFF corre en AWS)
# o a un usuario IAM cuyas credenciales usará el BFF.
resource "aws_iam_policy" "bff_sns_publish_policy" {
  name        = "${var.project_name}-bff-sns-publish-policy"
  description = "Permite al servicio BFF publicar mensajes en el topic SNS de emails."

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect   = "Allow",
        Action   = "sns:Publish",
        Resource = aws_sns_topic.email_notifications.arn
      }
    ]
  })
}

# 5. Rol para la Lambda del BFF
resource "aws_iam_role" "bff_lambda_role" {
  name = "${var.project_name}-bff-lambda-role"

  assume_role_policy = jsonencode({
    Version = "2012-10-17",
    Statement = [{
      Action = "sts:AssumeRole",
      Effect = "Allow",
      Principal = {
        Service = "lambda.amazonaws.com"
      }
    }]
  })
}

resource "aws_iam_role_policy_attachment" "bff_lambda_logs" {
  role       = aws_iam_role.bff_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}

resource "aws_iam_role_policy_attachment" "bff_lambda_sns" {
  role       = aws_iam_role.bff_lambda_role.name
  policy_arn = aws_iam_policy.bff_sns_publish_policy.arn
}

# Política para que el BFF pueda invocar el API Gateway
resource "aws_iam_policy" "bff_api_gateway_invoke_policy" {
  name        = "${var.project_name}-bff-api-gateway-invoke-policy"
  description = "Permite al BFF invocar el API Gateway de la tarea 1."

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Action = "execute-api:Invoke",
        Resource = "${aws_api_gateway_deployment.task1_api_deployment.execution_arn}/*"
      }
    ]
  })
}

resource "aws_iam_role_policy_attachment" "bff_lambda_api_gateway" {
  role       = aws_iam_role.bff_lambda_role.name
  policy_arn = aws_iam_policy.bff_api_gateway_invoke_policy.arn
}
