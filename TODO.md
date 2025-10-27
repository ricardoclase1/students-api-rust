# TODO List for Fixing 502 Error in Lambda Integration

## 1. Update src/main.rs for Proper Actix Integration
- [x] Modify function_handler to use lambda_http's Actix integration properly
- [x] Ensure requests are routed through the Actix app and handlers
- [x] Add proper error handling and logging

## 2. Update lambdas.tf for Database Configuration
- [x] Add DATABASE_URL = ":memory:" to the Lambda environment variables
- [x] Ensure SQLite uses in-memory database for Lambda

## 3. Package Migrations Correctly
- [x] Verify migrations/01_db_init.sql is included in the Lambda package
- [ ] Test that db.rs can read the migration file in Lambda environment

## 4. Testing and Verification
- [x] Build and deploy the updated Lambda
- [x] Test API endpoints via API Gateway - Still getting 500 error
- [ ] Check CloudWatch logs for panic messages or errors
- [ ] Verify database initialization works in Lambda
