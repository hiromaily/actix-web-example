#!/bin/bash

ENDPOINT=http://127.0.0.1:8080

## API TEST

echo "[health check]"
curl -w'\n' http://127.0.0.1:8080/api/v1/health

## Admin functionality
echo "====== Admin ======"

echo "[create user]"
curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"first_name": "John","last_name": "Doe","email": "john.doe@example.com","password": "password1234","is_admin": true}' http://127.0.0.1:8080/api/v1/admin/users

echo "[admin login error]"
curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"email":"foobar@zmail.com", "password":"xxxxxxxxxxx"}' http://127.0.0.1:8080/api/v1/admin/login

echo "[admin login success]"
curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"email":"john.doe@example.com", "password":"password1234"}' http://127.0.0.1:8080/api/v1/admin/login

echo "[user list]"
curl -w'\n' http://127.0.0.1:8080/api/v1/admin/users

echo "[get user id]"
userid=$(curl -s http://127.0.0.1:8080/api/v1/admin/users | jq '.[0].id')
echo user id: ${userid}

echo "[get user id: ${userid}]"
curl -w'\n' http://127.0.0.1:8080/api/v1/admin/users/${userid}

echo "[update user]"
curl -w'\n' -X PUT -H "Content-Type: application/json" -d '{"first_name": "John3","last_name": "Doe3","email": "john.doe3@example.com","password": "password12345","is_admin": true}' http://127.0.0.1:8080/api/v1/admin/users/${userid}

echo "[delete user]"
curl -w'\n' -X DELETE http://127.0.0.1:8080/api/v1/admin/users/${userid}

## App functionality
echo "====== App ======"

echo "[create user for app]"
curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"first_name": "Jackson","last_name": "Smith","email": "jackson.smith@example.com","password": "password1234","is_admin": false}' http://127.0.0.1:8080/api/v1/admin/users

echo "[get user id]"
userid=$(curl -s http://127.0.0.1:8080/api/v1/admin/users | jq '.[0].id')
echo user id: ${userid}

echo "[app login]"
curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"email":"jackson.smith@example.com", "password":"password1234"}' http://127.0.0.1:8080/api/v1/app/login

echo "[add todo]"
curl -w'\n' -X POST -H "Content-Type: application/json" -d '{"title":"Programming", "description":"sample program using actix web", "status": "pending"}' http://127.0.0.1:8080/api/v1/app/users/${userid}/todos

echo "[get user todo list]"
curl -w'\n' http://127.0.0.1:8080/api/v1/app/users/${userid}/todos

echo "[get todo id]"
todoid=$(curl -s http://127.0.0.1:8080/api/v1/app/users/${userid}/todos | jq '.[0].id')
echo todo id: ${todoid}

echo "[get user todo]"
curl -w'\n' http://127.0.0.1:8080/api/v1/app/users/${userid}/todos/${todoid}

echo "[update todo]"
curl -w'\n' -X PUT -H "Content-Type: application/json" -d '{"title":"Programming2", "description":"sample program using actix web 2", "status": "doing"}' http://127.0.0.1:8080/api/v1/app/users/${userid}/todos/1

echo "[delete todo]"
curl -w'\n' -X DELETE http://127.0.0.1:8080/api/v1/app/users/${userid}/todos/${todoid}
