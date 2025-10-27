

##########################################################################################
# 1. Topic de SNS para notificaciones de email
resource "aws_sns_topic" "email_notifications" {
  name = "${var.project_name}-email-notifications"
}
##########################################################################################
# 2. Cola de SQS para consumir los mensajes de email de forma asíncrona
resource "aws_sqs_queue" "email_queue" {
  name                       = "${var.project_name}-email-queue"
  visibility_timeout_seconds = 30 # Debe ser >= al timeout de la Lambda
}

# 3. Suscripción de la cola SQS al topic de SNS
resource "aws_sns_topic_subscription" "email_queue_subscription" {
  topic_arn = aws_sns_topic.email_notifications.arn
  protocol  = "sqs"
  endpoint  = aws_sqs_queue.email_queue.arn
}

# 4. Política para permitir que SNS envíe mensajes a la cola SQS
resource "aws_sqs_queue_policy" "allow_sns_to_send" {
  queue_url = aws_sqs_queue.email_queue.id

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Principal = {
          Service = "sns.amazonaws.com"
        },
        Action   = "sqs:SendMessage",
        Resource = aws_sqs_queue.email_queue.arn,
        Condition = {
          ArnEquals = {
            "aws:SourceArn" = aws_sns_topic.email_notifications.arn
          }
        }
      }
    ]
  })
}

# 5. Dead Letter Queue (DLQ) para la cola de emails
resource "aws_sqs_queue" "email_dlq" {
  name = "${var.project_name}-email-dlq"
}

# 6. Política para la DLQ
resource "aws_sqs_queue_policy" "allow_sns_to_send_dlq" {
  queue_url = aws_sqs_queue.email_dlq.id

  policy = jsonencode({
    Version = "2012-10-17",
    Statement = [
      {
        Effect = "Allow",
        Principal = {
          Service = "sqs.amazonaws.com"
        },
        Action   = "sqs:SendMessage",
        Resource = aws_sqs_queue.email_dlq.arn,
        Condition = {
          ArnEquals = {
            "aws:SourceArn" = aws_sqs_queue.email_queue.arn
          }
        }
      }
    ]
  })
}

# 7. Configurar redrive policy en la cola principal para usar DLQ
resource "aws_sqs_queue_redrive_policy" "email_queue_redrive" {
  queue_url = aws_sqs_queue.email_queue.id
  redrive_policy = jsonencode({
    deadLetterTargetArn = aws_sqs_queue.email_dlq.arn
    maxReceiveCount     = 3
  })
}
