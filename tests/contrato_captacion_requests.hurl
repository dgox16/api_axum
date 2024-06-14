POST http://localhost:8000/api/contrato_captacion/nuevo/
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "id_persona": 2,
    "cuenta": "12323213123",
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
