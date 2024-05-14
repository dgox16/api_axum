GET http://localhost:8000/api

# Se registra un usuario en la base de datos
POST http://localhost:8000/api/auth/registro
content-type: application/json

{
  "usuario": "dgox24",
  "email": "dgox24@gmail.com",
  "contraseña": "123"
}
 
# Se inicia sesion
POST http://localhost:8000/api/auth/inicio_sesion
content-type: application/json

{
  "usuario": "dgox24",
  "contraseña": "123"
}

# Se cierra sesion
GET http://localhost:8000/api/auth/cerrar_sesion
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxIiwiaWF0IjoxNzEzNDc0NTMxLCJleHAiOjE3MTM0NzgxMzF9.DB9y8FPRciGh5oaYlRPWNxpL9LowPXJZwEa5XWASZmE

# Se devuelve un usuario
GET http://localhost:8000/api/usuarios/actual
Authorization: Bearer {{token}}
