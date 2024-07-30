GET http://localhost:8000/api

# Se registra un usuario en la base de datos
POST http://localhost:8000/api/auth/registro
content-type: application/json

{
  "usuario": "dgox16",
  "email": "dgox24@gmail.com",
  "contraseña": "sheldon"
}
 
# Se inicia sesion
POST http://localhost:8000/api/auth/inicio_sesion
content-type: application/json

{
  "usuario": "dgox16",
  "contraseña": "sheldon"
}

# Se cierra sesion
GET http://localhost:8000/api/auth/cerrar_sesion
Authorization: Bearer {{token}}

# Se devuelve un usuario
GET http://localhost:8000/api/usuarios/actual
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOjMsImlhdCI6MTcxOTcwOTgyMCwiZXhwIjoxNzE5NzEzNDIwLCJwcm9wb3NpdG8iOiJBY2Nlc3NUb2tlbiJ9.Ncf6RonRbTxt-3QEOdifO-58UpWGkV80ON_SFx0xTOs
