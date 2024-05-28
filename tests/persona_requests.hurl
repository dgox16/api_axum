# Crear una nueva persona (Necesario tener una sesion iniciado)
POST http://localhost:8000/api/persona/nueva
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "nombre": "Pedro",
    "apellido_paterno": "Martinez",
    "apellido_materno": "Leon",
    "tipo": 13,
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
GET http://localhost:8000/api/persona/obtener/7
Authorization: Bearer {{token}}

# Buscar personas por tipo, nombre, apellidos
GET http://localhost:8000/api/persona/buscar/?tipo=referencia
Authorization: Bearer {{token}}

# Rellenar los datos de un aspirante 
GET http://localhost:8000/api/persona/aspirante/nuevo/6
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
GET http://localhost:8000/api/persona/socio/nuevo/9
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
GET http://localhost:8000/api/persona/aval/nuevo/9
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
GET http://localhost:8000/api/persona/menor/nuevo/11
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
GET http://localhost:8000/api/persona/conyuge/nuevo/12
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "fecha_residencia": "2020-12-16",
    "lugar_nacimiento": "Yuriria",
    "estado_nacimiento": 11,
    "regimen_conyugal": "BienesMancomunados"
}

# Rellenar los datos de un cliente
GET http://localhost:8000/api/persona/cliente/nuevo/13
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
GET http://localhost:8000/api/persona/sucursal/nuevo/15
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
GET http://localhost:8000/api/persona/tercero_autorizado/nuevo/16
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
GET http://localhost:8000/api/persona/propietario_real/nuevo/19
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
GET http://localhost:8000/api/persona/proveedor_recursos/nuevo/17
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
GET http://localhost:8000/api/persona/tutor/nuevo/20
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
GET http://localhost:8000/api/persona/beneficiario/nuevo/23
Authorization: Bearer {{token}}
Content-Type: application/json

{
    "entre_calle": 10000000001,
    "y_calle": 10000000002
}

# Agregar contactos de una persona
GET http://localhost:8000/api/persona/contacto_persona/nuevo/7
Authorization: Bearer {{token}}
Content-Type: application/json

[
    {
        "contacto": "4451112061",
        "tipo": "TelefonoMovil",
        "es_principal": true
    }
]
