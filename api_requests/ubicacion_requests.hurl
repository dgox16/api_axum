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
GET http://localhost:8000/api/pais/buscar/

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
