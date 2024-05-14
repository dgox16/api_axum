# Crear una nueva persona (Necesario tener una sesion iniciado)
POST http://localhost:8000/api/persona/nueva
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "nombre": "Andrea Monserrat",
    "apellido_paterno": "Martinez",
    "apellido_materno": "Leon",
    "tipo": 1,
    "sexo": "Masculino",
    "cp": "38940",
    "barrio": 1,
    "ciudad": 10011,
    "calle": 10000000001,
    "numero_exterior": "17",
    "numero_interior": "A",
    "vivienda": "Propio",
    "geolocalizacion": "Geolocalizacion 1",
    "observaciones_geolocalizacion": "Observaciones 1",
    "fecha_nacimiento": "1999-12-16",
    "pais_nacimiento": 1,
    "estado_civil": "Soltero",
    "es_embargo_precautorio": true,
    "bloqueado_autoridad": true,
    "tercero_autorizado": 1
}

# Obtener una persona con su id
GET http://localhost:8000/api/persona/obtener/3
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiaWF0IjoxNzE1NjM5OTkzLCJleHAiOjE3MTU2NTc5OTN9.IOrqSCwYAI9x8VGD30WRaRlcVYhjx9lGBo997-TYzhE

# Obtener una persona con su id
GET http://localhost:8000/api/persona/buscar/?palabra_clave=mon
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiaWF0IjoxNzE1NjM5OTkzLCJleHAiOjE3MTU2NTc5OTN9.IOrqSCwYAI9x8VGD30WRaRlcVYhjx9lGBo997-TYzhE
