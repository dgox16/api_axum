POST http://localhost:8000/api/contrato_captacion/nuevo/
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "id_persona": 2,
    "cuenta": "1232321312312345",
    "servicio": 1020,
    "fecha": "2024-6-14",
    "no_dejar_retirar_antes_de": "2025-6-14",
    "fecha_contrato": "2026-6-15",
    "monto_autorizado": 10000.43,
    "numero_sesion": "1",
    "tipo_sesion": "tipo 1",
    "nombre": "Garantia Liquida",
    "tipo": "PersonaFisica",
    "contrato_migrado": 100,
    "fecha_desbloqueo": "2025-2-22T20:18:14",
    "usuario_desbloqueo": 1,
    "fecha_libera_garantia": "2028-02-23T20:18:14",
    "usuario_libera_garantia": 1,
    "monto_libera_garantia": 5000.215,
    "tercero_autorizado": true,
    "tasa_pactada": 10.33,
    "bloqueada": false,
    "monto_bloqueado_adicional": 5000.215,
    "fecha_interes": "2024-6-14",
    "autoriza_cancelacion": true,
    "usuario_autoriza_cancelacion": 1
}

GET http://localhost:8000/api/contrato_captacion/listar/?persona=2
Authorization: Bearer {{token}}

GET http://localhost:8000/api/contrato_captacion/obtener_saldo/?persona=2&tipo=Cargos
Authorization: Bearer {{token}}

GET http://localhost:8000/api/contrato_captacion/obtener_temporales/?persona=2&tipo=Todos
Authorization: Bearer {{token}}

POST http://localhost:8000/api/contrato_captacion/abono_cargo/
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "persona": 2,
    "captacion": 6,
    "abono": 340.11,
    "cargo": 300.31,
    "abono_cargo": "Abono"
}

POST http://localhost:8000/api/contrato_captacion/depositar/
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "folio_ficha": "321321312",
    "persona": 2,
    "sucursal": 2,
    "poliza": 1,
    "operacion_fuente": 1,
    "tipo_pago": "Efectivo",
    "factura": 1,
    "subficha": 1,
    "procesado":1
}

POST http://localhost:8000/api/contrato_captacion/retirar/
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "folio_ficha": "321321312",
    "persona": 2,
    "sucursal": 2,
    "poliza": 1,
    "operacion_fuente": 1,
    "tipo_pago": "Transferencia",
    "factura": 1,
    "subficha": 1,
    "procesado":1
}

