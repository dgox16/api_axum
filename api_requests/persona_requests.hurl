POST http://localhost:8000/api/persona/nueva
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiaWF0IjoxNzE0NTg1MDMzLCJleHAiOjE3MTQ2MDMwMzN9.CAi3-uYNyYC-2dYN2BkuOgUivWy0EPquj_LbKnR6Hrg
Content-Type: application/json

{
    "nombre": "Diego",
    "apellido_paterno": "Gomez",
    "apellido_materno": "Martinez",
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
