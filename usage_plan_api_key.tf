# 1. Etapa de despliegue (ej. 'v1')
resource "aws_api_gateway_stage" "api_stage" {
  deployment_id = aws_api_gateway_deployment.task1_api_deployment.id
  rest_api_id   = aws_api_gateway_rest_api.task1_api_gateway.id
  stage_name    = "v1"
}

# 2. Clave de API que usará el BFF
resource "aws_api_gateway_api_key" "bff_api_key" {
  name    = "${var.project_name}-bff-key"
  enabled = true
}

# 3. Plan de uso para asociar la clave a la API
resource "aws_api_gateway_usage_plan" "api_usage_plan" {
  name = "${var.project_name}-usage-plan"

  api_stages {
    api_id = aws_api_gateway_rest_api.task1_api_gateway.id
    stage  = aws_api_gateway_stage.api_stage.stage_name
  }

  # Opcional: Configurar límites de throttling
  # throttle {
  #   burst_limit = 20
  #   rate_limit  = 10
  # }
}

# 4. Asociar la clave de API con el plan de uso
resource "aws_api_gateway_usage_plan_key" "main" {
  key_id        = aws_api_gateway_api_key.bff_api_key.id
  key_type      = "API_KEY"
  usage_plan_id = aws_api_gateway_usage_plan.api_usage_plan.id
}

# NOTA: Para que la API Key sea obligatoria, cada método en API Gateway
# debe tener "API Key Required" = true. AWS_PROXY no lo permite a nivel de recurso,
# pero se puede configurar en la consola o con una especificación OpenAPI.
# Por simplicidad, aquí creamos la clave y el plan; el BFF la enviará.