#!/bin/bash

echo "== Register =="
curl -X POST http://localhost:8080/api/register \
    -H "Content-Type: application/json" \
    -d '{"email": "fold.func@gmail.com", "password": "pass2331"}'

echo -e "\n== Login =="
curl -X POST http://localhost:8080/api/login \
  -H "Content-Type: application/json" \
  -d '{"email": "fold.func@gmail.com", "password": "pass2331"}' \
  -c cookies.txt

echo -e "\n== Logout =="
curl -X POST http://localhost:8080/api/logout \
  --cookie cookies.txt \
  --cookie-jar cookies.txt

