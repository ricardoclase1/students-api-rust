provider "aws" {
  region = var.aws_region
}
##########################################################################################
# 2. Rol de IAM: Define los permisos que tendrá nuestra función Lambda.
resource "aws_iam_role" "students_api_lambda_role" {
  name = "students-api-lambda-role"

  # Política de confianza: Permite que el servicio Lambda de AWS asuma este rol.
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
##########################################################################################
# 3. Adjuntar Política de IAM: Le damos al rol los permisos básicos de ejecución de Lambda.
# Esto es crucial para que la Lambda pueda escribir logs en CloudWatch.
resource "aws_iam_role_policy_attachment" "students_api_lambda_logs" {
  role       = aws_iam_role.students_api_lambda_role.name
  policy_arn = "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole"
}



# resource "aws_lambda_function" "students_api_lambda" { ... }
# resource "aws_api_gateway_rest_api" "students_api_rest_gateway" { ... }
# ... etc ...
