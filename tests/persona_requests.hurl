# Crear una nueva persona (Necesario tener una sesion iniciado)
POST http://localhost:8000/api/persona/nueva
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "nombre": "Pedro",
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
GET http://localhost:8000/api/persona/obtener/11
Authorization: Bearer {{token}}

# Buscar personas por tipo, nombre, apellidos
GET http://localhost:8000/api/persona/buscar/?tipo=aspirante
Authorization: Bearer {{token}}

# Rellenar los datos de un aspirante 
POST http://localhost:8000/api/persona/aspirante/nuevo/1
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "ocupacion_pld": 1, 
    "especificacion_pld": 26,
    "actividad_pld": 1078,
    "antiguedad": 2,
    "periodo": "Mensual",
    "frecuencia_captacion": 12,
    "operacion_maxima_captacion": 10.11,
    "perfil_frecuencia_prestamo": 10,
    "operacion_maxima_prestamo": 17.2,
    "ingresos_mensual": 32.2,
    "egresos_mensual": 32.2,
    "grado_afectacion": 32,
    "afectacion": 32.6,
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un socio
POST http://localhost:8000/api/persona/socio/nuevo/1
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "ocupacion_pld": 1, 
    "especificacion_pld": 26,
    "actividad_pld": 1078,
    "antiguedad": 2,
    "periodo": "Mensual",
    "frecuencia_captacion": 12,
    "operacion_maxima_captacion": 10.11,
    "perfil_frecuencia_prestamo": 10,
    "operacion_maxima_prestamo": 17.2,
    "ingresos_mensual": 32.2,
    "egresos_mensual": 32.2,
    "grado_afectacion": 32,
    "afectacion": 32.6,
    "proveedor_recursos": 2,
    "parentesco": 1,
    "persona_recomendo": 2,
    "manera_enterarse": 1,
    "lengua": 1,
    "empresa": 1,
    "puesto": 1,
    "fecha_trabajo": "2021-01-10",
    "ingreso_ordinario": 10.21,
    "otros_ingresos": 12.13,
    "es_propietario": "Si",
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un aval
POST http://localhost:8000/api/persona/aval/nuevo/9
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "socio_migrado": "Socio1",
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true,
    "fecha_bloqueo": "2025-12-16",
    "usuario_bloqueo": 1
}

# Rellenar los datos de un menor
POST http://localhost:8000/api/persona/menor/nuevo/11
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un conyuge
POST http://localhost:8000/api/persona/conyuge/nuevo/12
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,
    "regimen_conyugal": "BienesMancomunados"
}

# Rellenar los datos de un cliente
POST http://localhost:8000/api/persona/cliente/nuevo/13
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "ocupacion_pld": 1, 
    "especificacion_pld": 26,
    "actividad_pld": 1078,
    "antiguedad": 2,
    "periodo": "Mensual",
    "frecuencia_captacion": 12,
    "operacion_maxima_captacion": 10.11,
    "perfil_frecuencia_prestamo": 10,
    "operacion_maxima_prestamo": 17.2,
    "ingresos_mensual": 32.2,
    "egresos_mensual": 32.2,
    "grado_afectacion": 32,
    "afectacion": 32.6,
    "proveedor_recursos": 2,
    "parentesco": 1,
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un sucursal-persona
POST http://localhost:8000/api/persona/sucursal/nuevo/15
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un tercero autorizado
POST http://localhost:8000/api/persona/tercero_autorizado/nuevo/16
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "socio_migrado": "Socio1",
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true,
    "fecha_bloqueo": "2025-12-16",
    "usuario_bloqueo": 1
}

# Rellenar los datos de un propietario real
POST http://localhost:8000/api/persona/propietario_real/nuevo/19
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "ocupacion_pld": 1, 
    "especificacion_pld": 26,
    "actividad_pld": 1078,
    "antiguedad": 2,
    "periodo": "Mensual",
    "frecuencia_captacion": 12,
    "operacion_maxima_captacion": 10.11,
    "perfil_frecuencia_prestamo": 10,
    "operacion_maxima_prestamo": 17.2,
    "ingresos_mensual": 32.2,
    "egresos_mensual": 32.2,
    "grado_afectacion": 32,
    "afectacion": 32.6,
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un proveedor de recursos
POST http://localhost:8000/api/persona/proveedor_recursos/nuevo/17
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "ocupacion_pld": 1, 
    "especificacion_pld": 26,
    "actividad_pld": 1078,
    "antiguedad": 2,
    "periodo": "Mensual",
    "frecuencia_captacion": 12,
    "operacion_maxima_captacion": 10.11,
    "perfil_frecuencia_prestamo": 10,
    "operacion_maxima_prestamo": 17.2,
    "ingresos_mensual": 32.2,
    "egresos_mensual": 32.2,
    "grado_afectacion": 32,
    "afectacion": 32.6,
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un tutor
POST http://localhost:8000/api/persona/tutor/nuevo/20
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "clasificacion": "Fisica",
    "entre_calle": 10000000001,
    "y_calle": 10000000002,
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,   
    "regimen_conyugal": "BienesMancomunados",
    "profesion": 38,
    "escolaridad": 8,
    "autorizo_compartir_informacion_ifai": true,
    "autorizo_publicidad": true
}

# Rellenar los datos de un beneficiario
POST http://localhost:8000/api/persona/beneficiario/nuevo/23
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "entre_calle": 10000000001,
    "y_calle": 10000000002
}

# Agregar contactos de una persona
POST http://localhost:8000/api/persona/contacto_persona/nuevo/6
Authorization: Bearer {{token}}
Content-Type: application/json

[
    {
        "contacto": "4451112061",
        "tipo": "TelefonoMovil",
        "es_principal": true
    }
]

# Agregar documentos de una persona
POST http://localhost:8000/api/persona/documento_persona/nuevo/6
Authorization: Bearer {{token}}
Content-Type: application/json

[
    {
        "documento": 1,
        "numero_identificacion": "GOMD991216HGTMRG01",
        "es_identificacion": true,
        "vigencia": "2027-12-16"
    },
    {
        "documento": 2,
        "numero_identificacion": "GOMD9912168L8",
        "es_identificacion": true,
        "vigencia": "2027-12-16"
    }
]

# Agregar documentos de una persona
POST http://localhost:8000/api/persona/relacion_persona/nuevo/7
Authorization: Bearer {{token}}
Content-Type: application/json

[
    {
        "persona_relacionada": 2,
        "parentesco": 1
    },
    {
        "persona_relacionada": 3,
        "parentesco": 3
    }
]

POST http://localhost:8000/api/persona/tutor_persona/nuevo/11
Authorization: Bearer {{token}}
Content-Type: application/json

[
    {
        "tutor": 20,
        "tutor_migrado": "tutor migrado 1",
        "quien_es_tutor": "Madre",
        "documento_legal": "documento 1",
        "documento": 1,
        "tipo": "Principal"
    },
    {
        "tutor": 20,
        "tutor_migrado": "tutor migrado 2",
        "quien_es_tutor": "Madre",
        "documento_legal": "documento 2",
        "documento": 1,
        "tipo": "Principal"
    }
]
