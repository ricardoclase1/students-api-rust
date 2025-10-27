powershell
# --- CONFIGURACIÓN: MODIFICA SOLAMENTE LA LÍNEA DE ABAJO ---
$ProjectName = "students_api"
$LambdaFunctionName = "students-api-task1-api"   # REEMPLAZA ESTO con el nombre que encontraste en AWS.
$AWSRegion = "us-east-1"

# --- NO NECESITAS MODIFICAR NADA DE AQUÍ PARA ABAJO ---

# Paso 1: Compilar el proyecto en modo release para Linux
Write-Host "Paso 1: Compilando el proyecto en modo release para Lambda..."
cargo build --release --target x86_64-unknown-linux-musl

# Verificar si la compilación fue exitosa
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: La compilación ha fallado."
    exit 1
}

# Paso 2: Preparar y empaquetar los archivos para el despliegue
Write-Host "Paso 2: Creando el paquete de despliegue .zip..."
$BuildPath = "target/x86_64-unknown-linux-musl/release"
$ZipFile = "deployment.zip"

# Renombrar el binario compilado para Linux a 'bootstrap' (no tiene extensión .exe)
Copy-Item "$BuildPath/$ProjectName" -Destination "./bootstrap" -Force

# Crear el archivo .zip con el ejecutable
# Nota: Asumimos que no se usa SQLite, por lo que no se incluye 'students.db'
Compress-Archive -Path "./bootstrap" -DestinationPath $ZipFile -Force

# Verificar si el .zip se creó
if (-not (Test-Path $ZipFile)) {
    Write-Host "ERROR: No se pudo crear el archivo $ZipFile."
    Remove-Item "./bootstrap" # Limpieza
    exit 1
}

# Paso 3: Subir el nuevo código a AWS Lambda
Write-Host "Paso 3: Desplegando el nuevo código en la función Lambda '$LambdaFunctionName'..."
aws lambda update-function-code --function-name $LambdaFunctionName --zip-file "fileb://$ZipFile" --region $AWSRegion

# Verificar si el despliegue fue exitoso
if ($LASTEXITCODE -ne 0) {
    Write-Host "ERROR: El despliegue en AWS Lambda ha fallado."
} else {
    Write-Host "¡ÉXITO! El despliegue ha finalizado correctamente."
}

# Paso 4: Limpieza de archivos temporales
Write-Host "Paso 4: Limpiando archivos temporales..."
Remove-Item "./bootstrap"
Remove-Item $ZipFile

Write-Host "Proceso completado."