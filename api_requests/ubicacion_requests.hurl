# Crear calle
POST http://localhost:8000/api/calle/nueva
content-type: application/json

{
  "nombre": "Contadero",
  "tipo": "Privada"
}

# Buscar calle
GET http://localhost:8000/api/calle/buscar/?palabra=privada

# Buscar pais
GET http://localhost:8000/api/pais/buscar/?nombre=me

# Buscar estado
GET http://localhost:8000/api/estado_mexico/buscar/?nombre=gua

# Buscar pais
GET http://localhost:8000/api/pais/buscar/?nombre=me

## Crear domicilio
POST http://localhost:8000/api/domicilio/nuevo
content-type: application/json

{
  "cp": "38940",
  "colonia": "Centro",
  "calle_id": 10000001474,
  "entre_calle_id": 10000001846,
  "y_calle_id": 10000002369,
  "numero_exterior": "18"
}

## Crear ciudad
POST http://localhost:8000/api/ciudad/nueva
content-type: application/json

{
    "clave_localidad": 1,
    "estado": 11,
    "municipio": 1,
    "nombre": "Malinche",
    "tipo": "Tipo1",
    "clasificacion": "Localidad",
    "numero_habitantes": 1234,
    "orden": 0,
    "cp": "38940"
}

## Crear barrio
POST http://localhost:8000/api/barrio/nuevo
content-type: application/json

{
    "ciudad": 10011,
    "nombre": "La Joya",
    "cp": "38940",
    "tipo_cp": 1,
    "sindicatura": 1,
    "tipo_zona": "Tipo1",
    "numero_habitantes": 1000,
    "indice_marginacion": "Medio",
    "ponderacion_5c": 10.1,
    "c_municipio": 1,
    "unico_asentamiento": 1
}

# Buscar ciudades
GET http://localhost:8000/api/ciudad/buscar/?nombre=ja&limite=5
