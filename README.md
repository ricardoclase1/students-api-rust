# Students API with BFF and Android App

A comprehensive serverless architecture for managing students with email notifications, built with Rust (Actix-Web), Axum (BFF), AWS Lambda, API Gateway, SNS, SQS, and an Android client app.

## Architecture Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Android App   │────│   BFF (Axum)    │────│ Students API    │
│   (Jetpack      │    │   Lambda        │    │ (Actix-Web)     │
│    Compose)     │    │                 │    │ Lambda          │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │                       │                       │
         ▼                       ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   API Gateway   │    │   API Gateway   │    │   SQLite DB     │
│   (HTTP)        │    │   (REST)        │    │   (Embedded)    │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                                       │
                                                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   SNS Topic     │────│   SQS Queue     │────│ Email Processor │
│                 │    │                 │    │ Lambda (Python) │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │
         ▼
┌─────────────────┐
│   CloudWatch    │
│   Logs          │
└─────────────────┘
```

## Components

### 1. Students API (Rust - Actix-Web)
- RESTful API for student management
- JWT authentication
- SQLite database (embedded)
- Deployed as AWS Lambda function

### 2. Backend for Frontend (BFF) (Rust - Axum)
- API Gateway for the Android app
- Publishes email notifications to SNS
- Calls the Students API for data operations
- Deployed as AWS Lambda function

### 3. Email Processor (Python)
- Processes email notifications from SQS
- Validates email addresses
- Simulates email sending (ready for SES integration)
- Deployed as AWS Lambda function

### 4. Android App (Kotlin - Jetpack Compose)
- Modern Android app with Material 3 design
- Sends email notifications via BFF
- Uses Retrofit for API calls
- MVVM architecture with ViewModels

### 5. Infrastructure (Terraform)
- Complete AWS infrastructure as code
- API Gateway, Lambda functions, SNS, SQS
- IAM roles and policies
- CloudWatch logging

## Features

- **Student Management**: CRUD operations for students
- **Email Notifications**: Asynchronous email sending via SNS/SQS
- **Authentication**: JWT-based auth for API access
- **Mobile App**: Android client for sending notifications
- **Monitoring**: CloudWatch logs for all components
- **Error Handling**: Dead Letter Queues for failed messages

## Prerequisites

- Rust (latest stable)
- Android Studio (for Android app)
- Terraform
- AWS CLI configured
- Docker (for building Lambda packages)

## Deployment

### 1. Build and Deploy Lambda Functions

```bash
# Build the Students API Lambda
cd src
cargo build --release --target x86_64-unknown-linux-musl
zip -j deployment.zip target/x86_64-unknown-linux-musl/release/students_api

# Build the BFF Lambda
cd ../bff
cargo build --release --target x86_64-unknown-linux-musl
zip -j ../bff-deployment.zip target/x86_64-unknown-linux-musl/release/bff

# Package Python Lambda
cd ..
zip -r lambdas/email-processor/email_processor.zip email_processor.py
```

### 2. Deploy Infrastructure

```bash
terraform init
terraform plan
terraform apply
```

### 3. Update Android App Configuration

Update `RetrofitClient.kt` with the actual BFF API Gateway URL from Terraform outputs.

### 4. Build Android App

```bash
cd android
./gradlew build
```

## API Endpoints

### Students API
- `GET /students` - List all students
- `POST /students` - Create student
- `GET /students/{id}` - Get student by ID
- `PUT /students/{id}` - Update student
- `DELETE /students/{id}` - Delete student
- `POST /login` - Authenticate user

### BFF API
- `POST /notify/email` - Send email notification

## Environment Variables

### Students API Lambda
- `JWT_SECRET` - Secret key for JWT tokens

### BFF Lambda
- `API_GATEWAY_URL` - Students API Gateway URL
- `API_KEY` - API key for Students API
- `SNS_TOPIC_ARN` - SNS topic ARN for notifications
- `AWS_REGION` - AWS region

## Monitoring

All components log to CloudWatch. Check the following log groups:
- `/aws/lambda/students-api-task1-api`
- `/aws/lambda/students-api-bff`
- `/aws/lambda/students-api-email-processor`
- `/aws/apigateway/students-api-bff-api`

## Security

- API Gateway requires API keys
- JWT authentication for sensitive operations
- IAM roles with least privilege
- VPC configuration recommended for production

## Development

### Local Testing

```bash
# Test Students API
cargo test

# Test BFF
cd bff
cargo test

# Run Android app in emulator
cd android
./gradlew installDebug
```

### Adding New Features

1. Update the Rust services with new endpoints
2. Update Terraform configuration
3. Update Android app UI and API calls
4. Test locally and deploy

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with tests
4. Submit a pull request

## License

This project is licensed under the MIT License.
