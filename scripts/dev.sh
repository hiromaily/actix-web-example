#!/bin/bash

ENDPOINT=http://127.0.0.1:8080

## API TEST

## Admin functionality
# echo "====== Admin ======"

# echo "[create user]"
# curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"first_name": "John","last_name": "Doe","email": "john.doe@example.com","password": "password1234","is_admin": true}' http://127.0.0.1:8080/api/v1/admin/users

# echo "[admin login success and get jwt token]"
# token=$(curl -s curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"email":"john.doe@example.com", "password":"password1234"}' http://127.0.0.1:8080/api/v1/admin/login | jq -r '.token')
# echo token: ${token}

# # TODO: use jwt token
# echo "[user list]"
# curl -w'\n' -H 'Accept: application/json' -H "Authorization: Bearer ${token}" http://127.0.0.1:8080/api/v1/admin/users
