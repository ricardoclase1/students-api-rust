# 1. API Gateway REST
resource "aws_api_gateway_rest_api" "task1_api_gateway" {
  name        = "${var.project_name}-api-gateway"
  description = "API Gateway para el backend de la Tarea 1"

  endpoint_configuration {
    types = ["REGIONAL"]
  }
}

# 2. Recurso proxy para capturar todas las rutas (e.g., /students, /login)
resource "aws_api_gateway_resource" "proxy" {
  rest_api_id = aws_api_gateway_rest_api.task1_api_gateway.id
  parent_id   = aws_api_gateway_rest_api.task1_api_gateway.root_resource_id
  path_part   = "{proxy+}"
}

# 3. Método ANY para el recurso proxy. Aquí es donde forzamos la API Key.
resource "aws_api_gateway_method" "proxy_any" {
  rest_api_id   = aws_api_gateway_rest_api.task1_api_gateway.id
  resource_id   = aws_api_gateway_resource.proxy.id
  http_method   = "ANY"
  authorization = "NONE"
  api_key_required = true
}

# 3. Integración de la Lambda con el API Gateway
resource "aws_api_gateway_integration" "lambda_proxy_integration" {
  rest_api_id = aws_api_gateway_rest_api.task1_api_gateway.id
  resource_id = aws_api_gateway_resource.proxy.id
  http_method = "ANY"

  integration_http_method = "POST"
  type                    = "AWS_PROXY"
  uri                     = aws_lambda_function.task1_api_lambda.invoke_arn
}

# 4. Despliegue del API Gateway
resource "aws_api_gateway_deployment" "task1_api_deployment" {
  rest_api_id = aws_api_gateway_rest_api.task1_api_gateway.id

  triggers = {
    redeployment = sha1(jsonencode([
      aws_api_gateway_resource.proxy.id,
      aws_api_gateway_method.proxy_any.id,
      aws_api_gateway_integration.lambda_proxy_integration.id,
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }
}

# 5. API Gateway HTTP para el BFF
resource "aws_apigatewayv2_api" "bff_http_api" {
  name          = "${var.project_name}-bff-api"
  description   = "HTTP API Gateway para el BFF"
  protocol_type = "HTTP"
}

# 6. Integración del BFF con el API Gateway HTTP
resource "aws_apigatewayv2_integration" "bff_lambda_integration" {
  api_id           = aws_apigatewayv2_api.bff_http_api.id
  integration_type = "AWS_PROXY"

  connection_type    = "INTERNET"
  integration_method = "POST"
  integration_uri    = aws_lambda_function.bff_lambda.invoke_arn
}

# 7. Ruta para el BFF API
resource "aws_apigatewayv2_route" "bff_route" {
  api_id    = aws_apigatewayv2_api.bff_http_api.id
  route_key = "POST /notify/email"
  target    = "integrations/${aws_apigatewayv2_integration.bff_lambda_integration.id}"
}

# 8. Despliegue del API Gateway HTTP para BFF
resource "aws_apigatewayv2_deployment" "bff_deployment" {
  api_id      = aws_apigatewayv2_api.bff_http_api.id
  description = "Deployment for BFF API"

  triggers = {
    redeployment = sha1(jsonencode([
      aws_apigatewayv2_route.bff_route.id,
      aws_apigatewayv2_integration.bff_lambda_integration.id,
    ]))
  }

  lifecycle {
    create_before_destroy = true
  }
}

# 9. Stage para el BFF API
resource "aws_apigatewayv2_stage" "bff_stage" {
  api_id        = aws_apigatewayv2_api.bff_http_api.id
  name          = "prod"
  deployment_id = aws_apigatewayv2_deployment.bff_deployment.id

  access_log_settings {
    destination_arn = aws_cloudwatch_log_group.bff_api_logs.arn
    format = jsonencode({
      requestId               = "$context.requestId"
      sourceIp                = "$context.identity.sourceIp"
      requestTime             = "$context.requestTime"
      protocol                = "$context.protocol"
      httpMethod              = "$context.httpMethod"
      resourcePath            = "$context.resourcePath"
      routeKey                = "$context.routeKey"
      status                  = "$context.status"
      responseLength          = "$context.responseLength"
      integrationErrorMessage = "$context.integrationErrorMessage"
    })
  }
}

# 10. Permiso para que el API Gateway invoque la Lambda del BFF
resource "aws_lambda_permission" "bff_api_gateway_invoke" {
  statement_id  = "AllowAPIGatewayInvoke"
  action        = "lambda:InvokeFunction"
  function_name = aws_lambda_function.bff_lambda.function_name
  principal     = "apigateway.amazonaws.com"
  source_arn    = "${aws_apigatewayv2_api.bff_http_api.execution_arn}/*/*"
}