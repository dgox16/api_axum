# Crear poliza  (Necesario tener una cuenta iniciada)
POST http://localhost:8000/api/poliza/nueva
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiaWF0IjoxNzE0NTg1MDMzLCJleHAiOjE3MTQ2MDMwMzN9.CAi3-uYNyYC-2dYN2BkuOgUivWy0EPquj_LbKnR6Hrg
Content-Type: application/json

{
  "tipo": "Diario",
  "sucursal": 1,
  "concepto": "Concepto 11",
  "numero":20
}

# Crear poliza junto a detalles  (Necesario tener una cuenta iniciada)
POST http://localhost:8000/api/poliza/nueva
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiaWF0IjoxNzE0NTg1MDMzLCJleHAiOjE3MTQ2MDMwMzN9.CAi3-uYNyYC-2dYN2BkuOgUivWy0EPquj_LbKnR6Hrg
Content-Type: application/json

{
  "tipo": "Diario",
  "sucursal": 1,
  "concepto": "Concepto 14",
  "numero":20,
  "detalles_poliza":[
    {
      "poliza":1,
      "cuenta":3,
      "sucursal": 1,
      "cargo": 1.00,
      "abono": 2.00,
      "proveedor":1,
      "concepto": "concepto 1",
      "iva": "Iva11"
    },
    {
      "poliza":1,
      "cuenta":3,
      "sucursal": 1,
      "cargo": 1.00,
      "abono": 2.00,
      "proveedor":1,
      "concepto": "concepto 2"
    }
  ]
}

# Crear poliza de tipo egreso  (Necesario tener una cuenta iniciada y enviar los datos de una poliza de egreso)
POST http://localhost:8000/api/poliza/nueva
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiaWF0IjoxNzE0NTg1MDMzLCJleHAiOjE3MTQ2MDMwMzN9.CAi3-uYNyYC-2dYN2BkuOgUivWy0EPquj_LbKnR6Hrg
Content-Type: application/json

{
  "tipo": "Egreso",
  "sucursal": 1,
  "concepto": "Concepto 11",
  "fuente": "Pasiva",
  "poliza_egreso": {
    "beneficiario": "Beneficiario 4",
    "banco": 1,
    "cheque": "Cheque 1"
  }
}

# Crear poliza de tipo egreso con detalles (Necesario tener una cuenta iniciada y enviar los datos de una poliza de egreso)
POST http://localhost:8000/api/poliza/nueva
Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIyIiwiaWF0IjoxNzE0NTg1MDMzLCJleHAiOjE3MTQ2MDMwMzN9.CAi3-uYNyYC-2dYN2BkuOgUivWy0EPquj_LbKnR6Hrg
Content-Type: application/json

{
  "tipo": "Egreso",
  "sucursal": 1,
  "concepto": "Concepto 11",
  "fuente": "Pasiva",
  "poliza_egreso": {
    "beneficiario": "Beneficiario 4",
    "banco": 1,
    "cheque": "Cheque 1"
  },
  "detalles_poliza":[
    {
      "poliza":1,
      "cuenta":3,
      "sucursal": 1,
      "cargo": 1.00,
      "abono": 2.00,
      "proveedor":1,
      "concepto": "concepto 1",
      "iva": "Iva11"
    },
    {
      "poliza":1,
      "cuenta":3,
      "sucursal": 1,
      "cargo": 1.00,
      "abono": 2.00,
      "proveedor":1,
      "concepto": "concepto 2"
    }
  ]
}

# Buscar polizas por concepto y con la posibilidad de escoger el limite 
GET http://localhost:8000/api/poliza/buscar/concepto?limite=2&concepto=11

# Obtienes la poliza, su poliza de egreso en caso de existir y sus detalles de polizas
GET http://localhost:8000/api/poliza/obtener/30
