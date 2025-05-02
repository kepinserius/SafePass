#!/bin/bash

echo "Testing SafePass API"
echo "-------------------"

# Register user
echo -e "\n1. Registering user..."
REGISTER_RESPONSE=$(curl -v -X POST http://127.0.0.1:8080/api/user/register \
  -H "Content-Type: application/json" \
  -d '{"name": "Test User", "email": "test@example.com", "password": "Password123!"}')

echo "Raw response: $REGISTER_RESPONSE"

# Login
echo -e "\n2. Logging in..."
LOGIN_RESPONSE=$(curl -v -X POST http://127.0.0.1:8080/api/user/login \
  -H "Content-Type: application/json" \
  -d '{"email": "test@example.com", "password": "Password123!"}')

echo "Raw response: $LOGIN_RESPONSE"

# Try to extract token manually
echo "Please look at the raw response and extract the token manually if it's available"
echo "Then set it with: TOKEN=your_token_here"
TOKEN=""

# Manual test a few endpoints
echo -e "\n3. Testing individual endpoints manually:"
echo "When you have the token, run:"
echo "curl -v -X GET http://127.0.0.1:8080/api/user/profile -H \"Authorization: Bearer \$TOKEN\""
echo "curl -v -X POST http://127.0.0.1:8080/api/passwords -H \"Authorization: Bearer \$TOKEN\" -H \"Content-Type: application/json\" -d '{\"site_name\": \"GitHub\", \"site_url\": \"https://github.com\", \"username\": \"myusername\", \"password\": \"SecretPass123!\", \"notes\": \"My GitHub account\"}'"

echo -e "\nTesting complete! Check the raw responses for details."