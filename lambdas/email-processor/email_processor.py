import json
import logging
import boto3
from email_validator import validate_email, EmailNotValidError

# Configurar logging
logger = logging.getLogger()
logger.setLevel(logging.INFO)

def lambda_handler(event, context):
    """
    Lambda function to process email notifications from SQS.
    Validates email addresses and simulates sending emails.
    """
    sqs = boto3.client('sqs')

    for record in event['Records']:
        try:
            # Parsear el mensaje de SQS
            message_body = json.loads(record['body'])
            email_data = json.loads(message_body['Message'])

            email = email_data.get('email')
            subject = email_data.get('subject', 'Notification')
            body = email_data.get('body', 'This is a notification.')

            logger.info(f"Processing email for: {email}")

            # Validar el email
            try:
                valid = validate_email(email)
                email = valid.email
                logger.info(f"Email validated: {email}")
            except EmailNotValidError as e:
                logger.error(f"Invalid email address: {email} - {str(e)}")
                # En un escenario real, podríamos enviar a una DLQ o notificar
                continue

            # Simular envío de email (en producción usarías SES, SendGrid, etc.)
            logger.info(f"Simulating email send to {email} with subject '{subject}'")

            # Aquí iría el código real para enviar el email
            # Por ejemplo, usando boto3 SES:
            # ses = boto3.client('ses')
            # ses.send_email(
            #     Source='noreply@yourdomain.com',
            #     Destination={'ToAddresses': [email]},
            #     Message={
            #         'Subject': {'Data': subject},
            #         'Body': {'Text': {'Data': body}}
            #     }
            # )

            # Marcar el mensaje como procesado (borrarlo de la cola)
            # Esto se hace automáticamente por SQS si la función no lanza excepción

        except Exception as e:
            logger.error(f"Error processing message: {str(e)}")
            # Si hay error, el mensaje volverá a la cola para reintento
            # Después de maxReceiveCount intentos, irá a la DLQ
            raise e

    return {
        'statusCode': 200,
        'body': json.dumps('Email processing completed successfully')
    }
