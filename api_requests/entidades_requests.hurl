# Crear banco
POST http://localhost:8000/api/banco/nuevo
content-type: application/json

{
  "nombre": "BBVA"
}

# Crear sucursal
POST http://localhost:8000/api/sucursal/nueva
content-type: application/json

{
  "nombre": "Sucursal 3",
  "encargado": 2,
  "domicilio": 1  
}

# Crear Proveedor
POST http://localhost:8000/api/proveedor/nuevo
content-type: application/json

{
  "nombre": "Proveedor 1",
  "domicilio": 1,
  "rfc":"GOMD991216123",
  "curp": "GOMD991216HGT12331",
  "telefono": "4451312061",
  "tipo": "Global",
  "regimen": "Regimen 1",
  "banco": 1,
  "cuenta_clabe": "ASDASDASDAS"
}

# Crear una cuenta
POST http://localhost:8000/api/cuenta/nueva
Content-Type: application/json

{
  "cuenta": "123213412",
  "cuenta_siti": "c_siti1",
  "nombre": "cuenta 2 ",
  "clasificacion": "Capitulo",
  "padre": "padre 1",
  "nivel": 1,
  "en_catalogo_minimo": true,
  "nombre_balance": "nombre balance 1",
  "nombre_siti": "nombre siti 1",
  "cuenta_padre_siti": "cuenta padre siti 1",
  "cuenta_agrupar": "cuenta agrupar 1",
  "orden_siti": 123,
  "prorrateo": true
}
