CREATE TYPE estado_prestamo AS ENUM (
    'solicitud',
    'aprobado',
    'entregado',
    'vigente',
    'pagado',
    'renovado',
    'cartera_vencida',
    'castigado',
    'cancelada'
);

CREATE TYPE etapa_prestamo AS ENUM (
    'solicitud',
    'investigacion',
    'analisis',
    'autorizacion',
    'dispersion',
    'instrumentacion',
    'mesa_control',
    'entrega',
    'entregado',
    'linea_activa',
    'autorizacion_legal',
    'legal',
    'proceso_castigo',
    'castigado'
);

CREATE TYPE ministracion_prestamo AS ENUM (
    'una_ministracion',
    'ministracion_variable'
);

CREATE TYPE tipo_pagares_prestamo AS ENUM (
    'capital_igual',
    'pagare_igual',
    'al_vencimiento'
);


CREATE TABLE prestamos_fecha (
    id_prestamo_fecha SERIAL PRIMARY KEY,
    fecha_solicitud TIMESTAMP NOT NULL,
    fecha_autorizacion DATE NOT NULL,
    fecha_entrega TIMESTAMP NOT NULL,
    fecha_vencimiento TIMESTAMP NOT NULL,
    fecha_vencimiento_linea DATE NOT NULL,
    fecha_interes DATE NOT NULL,
    fecha_interes_moratorio DATE NOT NULL,
    fecha_primer_incumplimiento DATE NOT NULL,
    fecha_primer_pagare DATE NOT NULL,
    fecha_ultima_ministracion DATE NOT NULL,
    fecha_antes_interes DATE NOT NULL,
    fecha_antes_moratorio DATE NOT NULL,
    fecha_devolucion_garantia DATE NOT NULL,
    fecha_ultima_investigacion DATE NOT NULL,
    estado estado_prestamo NOT NULL,
    etapa etapa_prestamo NOT NULL,
    ministracion ministracion_prestamo NOT NULL,
    tipo_de_pagares tipo_pagares_prestamo NOT NULL,
    monto_abono REAL NOT NULL,
    fecha_ajustar_primer_pagare DATE NOT NULL
);
