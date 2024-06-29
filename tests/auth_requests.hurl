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
  "usuario": "dagm",
  "contraseña": "sheldon"
}

# Se cierra sesion
GET http://localhost:8000/api/auth/cerrar_sesion
Authorization: Bearer {{token}}

# Se devuelve un usuario
GET http://localhost:8000/api/usuarios/actual
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImlhdCI6MTcxOTYzMjY1NywiZXhwIjoxNzE5NjM5ODU3LCJwcm9wb3NpdG8iOiJBY2Nlc3NUb2tlbiJ9.hgNXGuoAS80Lw8YMvtAjfRWVmf8EaExmG-PU_QAJf7M
