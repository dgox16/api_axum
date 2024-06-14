# Crear puesto de trabajo
POST http://localhost:8000/api/puesto_trabajo/nuevo/
Authorization: Bearer {{token}}
Content-Type: application/json

{
  "nombre": "Jefe"
}
