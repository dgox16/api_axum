CREATE TYPE clasificacion_actividad AS ENUM (
    'comercio',
    'consumo',
    'vivienda'
);

CREATE TABLE actividades (
    id_actividad SERIAL PRIMARY KEY,
    subsector INTEGER NOT NULL,
    rama INTEGER NOT NULL,
    nombre VARCHAR(100) NOT NULL,
    clasificacion clasificacion_actividad NOT NULL,
    aplica_iva BOOLEAN NOT NULL,
    activo BOOLEAN NOT NULL,
    actividad_fira INTEGER NOT NULL,
    cultivo BOOLEAN NOT NULL,
    ganado BOOLEAN NOT NULL,
    factor_riesgo INTEGER NOT NULL
);
INSERT INTO actividades (id_actividad, subsector, rama, nombre, clasificacion, aplica_iva, activo, actividad_fira, cultivo, ganado, factor_riesgo) VALUES
	(1, 1110, 11101, 'REFACCIONARIO AGRICOLA, COMPRA DE TERRENO ', 'comercio', false, true, 100, false, false, 3),
	(2, 1110, 11102, 'REFACCIONARIO AGRICOLA, COMPRA O MANTENIMIENTO DE TRACTOR O DE IMPLEMENTOS', 'comercio', false, true, 235, true, false, 3),
	(3, 1111, 11113, 'AVIO AGRICOLA, CULTIVO DE LEGUMBRES', 'comercio', false, false, 100, true, false, 3),
	(4, 1111, 11114, 'AVIO AGRICOLA, CULTIVO DE TRIGO ', 'comercio', false, false, 100, true, false, 3),
	(5, 1111, 11115, 'AVIO AGRICOLA, CULTIVO DE MAÍZ', 'comercio', false, false, 100, true, false, 3),
	(6, 1111, 11116, 'AVIO AGRICOLA, CULTIVO DE ARROZ', 'comercio', false, false, 100, true, false, 3),
	(7, 1111, 11119, 'AVIO AGRICOLA, CULTIVO DE OTROS GRANOS', 'comercio', false, false, 100, true, false, 3),
	(8, 1112, 11121, 'AVIO AGRICOLA, CULTIVO DE HORTALIZAS', 'comercio', false, false, 100, true, false, 3),
	(9, 1113, 11131, 'AVIO AGRICOLA, CULTIVO DE NARANJA', 'comercio', false, false, 100, true, false, 3),
	(10, 1113, 11132, 'AVIO AGRICOLA, CULTIVO DE OTROS CÍTRICOS', 'comercio', false, false, 100, true, false, 3),
	(11, 1113, 11133, 'AVIO AGRICOLA, CULTIVO DE FRUTALES NO CÍTRICOS ', 'comercio', false, false, 100, true, false, 3),
	(12, 1114, 11141, 'AVIO AGRICOLA, CULTIVO DE PRODUCTOS ALIMENTICIOS EN INVERNADEROS', 'comercio', false, false, 100, true, false, 2),
	(13, 1119, 11193, 'AVIO AGRICOLA, CULTIVO DE CAÑA DE AZÚCAR', 'comercio', false, false, 100, true, false, 3),
	(14, 1119, 11194, 'AVIO AGRICOLA, CULTIVOS DE ALFALFA Y PASTOS', 'comercio', false, false, 100, false, false, 3),
	(15, 1119, 11199, 'AVIO AGRICOLA, CULTIVOS VARIOS', 'comercio', false, false, 100, true, false, 3),
	(16, 1121, 11211, 'AVIO GANADERO, BOVINOS PARA CARNE (BECERROS DE ENGORDA)', 'comercio', false, false, 100, false, true, 3),
	(17, 1121, 11212, 'AVIO GANADERO, BOVINOS PARA LECHE (VACAS LECHERAS)', 'comercio', false, false, 100, false, true, 2),
	(18, 1122, 11221, 'AVIO GANADERO, PORCINOS (CERDOS)', 'comercio', false, false, 100, false, true, 3),
	(19, 1123, 11232, 'AVIO GANADERO, POLLO DE ENGORDA', 'comercio', false, false, 100, false, true, 3),
	(20, 1124, 11241, 'AVIO GANADERO, BORREGOS ', 'comercio', false, false, 100, false, true, 3),
	(21, 1125, 11251, 'REFACCIONARIO GANADERIA, CORRALETAS O INSTALACIONES ', 'comercio', false, true, 100, false, false, 3),
	(22, 2123, 21232, 'MINERÍA DE ARENA, GRAVA, ARCILLA Y OTROS MINERALES REFRACTARIOS', 'comercio', false, true, 100, false, false, 3),
	(23, 3118, 31181, 'ELABORACIÓN DE PAN Y OTROS PRODUCTOS DE PANADERÍA ', 'comercio', false, true, 200, false, false, 3),
	(24, 3118, 31183, 'ELABORACIÓN DE TORTILLAS DE MAÍZ Y MOLIENDA DE NIXTAMAL', 'comercio', false, true, 200, false, false, 3),
	(25, 3119, 31199, 'ELABORACIÓN DE ALIMENTOS PARA SU VENTA', 'comercio', false, true, 200, false, false, 3),
	(26, 3152, 31522, 'CONFECCIÓN DE ROPA', 'comercio', false, true, 200, false, false, 3),
	(27, 3118, 31182, 'FABRICACIÓN DE PRODUCTOS DE HERRERÍA', 'comercio', false, true, 200, false, false, 3),
	(28, 4311, 43111, 'COMERCIO, ABARROTES MAYOREO', 'comercio', false, true, 300, false, false, 3),
	(29, 4342, 43421, 'COMERCIO, MATERIALES PARA CONSTRUCCIÓN', 'comercio', false, true, 300, false, false, 3),
	(30, 4611, 46111, 'COMERCIO, ABARROTES MENUDEO', 'comercio', false, true, 300, false, false, 3),
	(31, 4611, 46112, 'COMERCIO, CARNICERIAS', 'comercio', false, true, 300, false, false, 3),
	(32, 4611, 46113, 'COMERCIO, FRUTAS Y VERDURAS', 'comercio', false, true, 300, false, false, 3),
	(33, 4611, 46115, 'COMERCIO, SEMILLAS Y GRANOS ALIMENTICIOS, ESPECIAS Y CHILES SECOS', 'comercio', false, true, 300, false, false, 3),
	(34, 4611, 46116, 'COMERCIO, PUESTOS AMBULANTES', 'comercio', false, true, 300, false, false, 3),
	(165, 1125, 1125102, 'REFACCIONARIO AGRICOLA, ESTABLECIMIENTO DE CULTIVOS PERENNES', 'comercio', false, true, 100, false, false, 3),
	(35, 4611, 46117, 'COMERCIO, DULCES Y MATERIAS PRIMAS PARA REPOSTERÍA', 'comercio', false, true, 300, false, false, 3),
	(36, 4612, 46121, 'COMERCIO, BEBIDAS', 'comercio', false, true, 300, false, false, 3),
	(37, 4615, 46151, 'SERVICIOS, AGENCIA DE VIAJES', 'comercio', false, true, 400, false, false, 3),
	(38, 4632, 46321, 'COMERCIO, ROPA Y ACCESORIOS DE VESTIR', 'comercio', false, true, 300, false, false, 3),
	(39, 4633, 46331, 'COMERCIO, ZAPATERíAS', 'comercio', false, true, 300, false, false, 3),
	(40, 4641, 46411, 'COMERCIO, PRODUCTOS FARMACÉUTICOS Y NATURISTAS', 'comercio', false, true, 300, false, false, 3),
	(41, 4651, 46511, 'COMERCIO, PERFUMERÍA Y JOYERÍA', 'comercio', false, true, 300, false, false, 3),
	(42, 4652, 46521, 'COMERCIO, ARTÍCULOS PARA EL ESPARCIMIENTO ', 'comercio', false, true, 300, false, false, 3),
	(43, 4653, 46531, 'COMERCIO, PAPELERÍA, LIBROS Y PERIÓDICOS', 'comercio', false, true, 300, false, false, 3),
	(44, 4659, 46591, 'COMERCIO, MASCOTAS, REGALOS, ARTÍCULOS RELIGIOSOS, ARTESANÍAS, ARTÍCULOS EN TIENDAS I', 'comercio', false, true, 300, false, false, 3),
	(45, 4661, 46611, 'COMERCIO, MUEBLES PARA EL HOGAR Y OTROS ENSERES DOMÉSTICOS', 'comercio', false, true, 300, false, false, 3),
	(46, 4662, 46621, 'COMERCIO, COMPUTADORAS, TELÉFONOS Y OTROS APARATOS DE COMUNICACIÓN', 'comercio', false, true, 300, false, false, 3),
	(47, 4663, 46631, 'COMERCIO, ARTÍCULOS PARA LA DECORACIÓN DE INTERIORES', 'comercio', false, true, 300, false, false, 3),
	(48, 4671, 46711, 'COMERCIO, FERRETERÍA, TLAPALERÍA Y VIDRIOS', 'comercio', false, true, 300, false, false, 3),
	(49, 4682, 46821, 'COMERCIO, PARTES Y REFACCIONES PARA AUTOMÓVILES, CAMIONETAS Y CAMIONES', 'comercio', false, true, 300, false, false, 2),
	(50, 4683, 46831, 'COMERCIO, MOTOCICLETAS Y OTROS VEHÍCULOS DE MOTOR', 'comercio', false, true, 300, false, false, 3),
	(51, 4684, 46841, 'COMERCIO, COMBUSTIBLES Y GASOLINERAS', 'comercio', false, true, 300, false, false, 3),
	(52, 4684, 46842, 'COMERCIO, ACEITES Y GRASAS LUBRICANTES, ADITIVOS Y SIMILARES', 'comercio', false, true, 300, false, false, 3),
	(53, 4851, 48511, 'SERVICIOS, TRANSPORTE TERRESTRE DE PASAJEROS Y TAXIS', 'comercio', false, true, 308, false, false, 3),
	(54, 5322, 53229, 'SERVICIOS, ALQUILER DE ARTÍCULOS PARA EL HOGAR', 'comercio', false, true, 400, false, false, 3),
	(55, 5411, 54111, 'SERVICIOS, BUFETES JURÍDICOS', 'comercio', false, true, 400, false, false, 3),
	(56, 5411, 54112, 'SERVICIOS, NOTARÍAS PÚBLICAS', 'comercio', false, true, 400, false, false, 3),
	(57, 5411, 54119, 'SERVICIOS, APOYO PARA EFECTUAR TRÁMITES LEGALES', 'comercio', false, true, 400, false, false, 3),
	(58, 5412, 54121, 'SERVICIOS, CONTABILIDAD, AUDITORÍA Y SERVICIOS RELACIONADOS', 'comercio', false, true, 400, false, false, 3),
	(59, 5413, 54131, 'SERVICIOS, ARQUITECTURA', 'comercio', false, true, 400, false, false, 3),
	(60, 5413, 54133, 'SERVICIOS, INGENIERÍA', 'comercio', false, true, 400, false, false, 3),
	(61, 5416, 54161, 'SERVICIOS, CONSULTORÍA ADMINISTRATIVA', 'comercio', false, true, 400, false, false, 3),
	(62, 5614, 56142, 'SERVICIOS, CASETAS TELEFÓNICAS, RECEPCIÓN DE LLAMADAS Y PROMOCIÓN POR TELÉFONO', 'comercio', false, true, 400, false, false, 3),
	(63, 5614, 56143, 'SERVICIOS, FOTOCOPIADO, FAX Y AFINES', 'comercio', false, true, 400, false, false, 3),
	(64, 6211, 62111, 'SERVICIOS, CONSULTORIOS MÉDICOS', 'comercio', false, true, 400, false, false, 1),
	(65, 6213, 62131, 'SERVICIOS, CONSULTORIOS QUIROPRÁCTICA', 'comercio', false, true, 400, false, false, 3),
	(66, 6213, 62133, 'SERVICIOS, CONSULTORIOS DE PSICOLOGÍA', 'comercio', false, true, 400, false, false, 3),
	(67, 6213, 62139, 'SERVICIOS, OTROS CONSULTORIOS PARA EL CUIDADO DE LA SALUD', 'comercio', false, true, 400, false, false, 3),
	(68, 6215, 62151, 'SERVICIOS, LABORATORIOS MÉDICOS Y DE DIAGNÓSTICO', 'comercio', false, true, 400, false, false, 1),
	(69, 6221, 62211, 'SERVICIOS, HOSPITALES GENERALES', 'comercio', false, true, 400, false, false, 3),
	(70, 6244, 62441, 'SERVICIOS, GUARDERÍAS', 'comercio', false, true, 400, false, false, 3),
	(71, 7111, 71112, 'SERVICIOS, COMPAÑÍAS DE DANZA', 'comercio', false, true, 400, false, false, 3),
	(72, 7111, 71113, 'SERVICIOS, CANTANTES Y GRUPOS MUSICALES', 'comercio', false, true, 400, false, false, 3),
	(73, 7139, 71394, 'SERVICIOS, CLUBES DEPORTIVOS, BALNEARIOS Y CENTROS DE ACONDICIONAMIENTO FÍSICO', 'comercio', false, true, 400, false, false, 3),
	(74, 7221, 72211, 'SERVICIOS, RESTAURANTES CON SERVICIOS DE MESEROS', 'comercio', false, true, 400, false, false, 3),
	(75, 7222, 72221, 'SERVICIOS, RESTAURANTES DE AUTOSERVICIOS Y DE COMIDA PARA LLEVAR', 'comercio', false, true, 400, false, false, 3),
	(76, 7224, 72241, 'SERVICIOS, CENTROS NOCTURNOS, BARES, CANTINAS Y SIMILARES', 'comercio', false, true, 400, false, false, 3),
	(77, 8111, 81111, 'SERVICIOS, TALLERES MECÁNICOS Y ELÉCTRICOS DE AUTOMÓVILES Y CAMIONES', 'comercio', false, true, 237, false, false, 2),
	(78, 8111, 81112, 'SERVICIOS, TALLERES DE HOJALATERÍA, PINTURA Y TAPICERÍA PARA AUTOS Y CAMIONES', 'comercio', false, true, 237, false, false, 3),
	(79, 8111, 81119, 'SERVICIOS, TALLERES OTROS SERVICIOS DE REPARACIÓN Y MANTENIMIENTO DE VEHICULOS', 'comercio', false, true, 237, false, false, 3),
	(80, 8112, 81121, 'SERVICIOS, REPARACIÓN Y MANTENIMIENTO DE EQUIPO ELECTRÓNICO Y DE EQUIPO DE PRECISIÓN', 'comercio', false, true, 237, false, false, 3),
	(81, 8114, 81141, 'SERVICIOS, REPARACIÓN Y MANTENIMIENTO DE APARATOS ELÉCTRICOS PARA EL HOGAR Y PERSONALES', 'comercio', false, true, 237, false, false, 2),
	(82, 8114, 81142, 'SERVICIOS, REPARACIÓN DE TAPICERÍA Y MUEBLES PARA EL HOGAR', 'comercio', false, true, 237, false, false, 3),
	(83, 8114, 81143, 'SERVICIOS, REPARACIÓN DE CALZADOS Y OTROS ARTÍCULOS DE PIEL Y CUERO', 'comercio', false, true, 237, false, false, 3),
	(84, 8114, 81149, 'SERVICIOS, REPARACIÓN Y MANTENIMIENTO DE OTROS ARTÍCULOS PARA EL HOGAR Y PERSONALES', 'comercio', false, true, 237, false, false, 2),
	(85, 8121, 81211, 'SERVICIOS, SALONES DE BELLEZA, ESTETICAS Y PELUQUERIAS', 'comercio', false, true, 400, false, false, 3),
	(86, 8122, 81221, 'SERVICIOS, LAVANDERÍAS Y TINTORERÍAS', 'comercio', false, true, 400, false, false, 3),
	(87, 8123, 81231, 'SERVICIOS, SERVICIOS FUNERARIOS', 'comercio', false, true, 400, false, false, 3),
	(88, 8124, 81241, 'SERVICIOS, ESTACIONAMIENTO Y PENSIONES PARA AUTOMÓVILES', 'comercio', false, true, 400, false, false, 2),
	(89, 8129, 81291, 'SERVICIOS, FOTO ESTUDIOS', 'comercio', false, true, 400, false, false, 3),
	(90, 6211, 621121, 'SERVICIOS, CONSULTORIOS DENTALES', 'comercio', false, true, 400, false, false, 3),
	(91, 7211, 721111, 'SERVICIOS, HOTELES Y MOTELES', 'comercio', false, true, 400, false, false, 3),
	(92, 9911, 991101, 'VIVIENDA, TRÁMITES LEGALES', 'vivienda', false, true, 0, false, false, 2),
	(93, 9911, 991102, 'VIVIENDA, SERVICIOS PROFESIONALES PARA INICIOS DE TRABAJOS', 'vivienda', false, true, 400, false, false, 2),
	(94, 9911, 991103, 'VIVIENDA, EXCAVACION Y CIMENTACION ', 'vivienda', false, true, 400, false, false, 2),
	(95, 9911, 991104, 'VIVIENDA, CONSTRUCCION DE VIVIENDA NUEVA', 'vivienda', false, true, 400, false, false, 2),
	(96, 9911, 991105, 'VIVIENDA, ALBAÑILERIA OBRA NEGRA', 'vivienda', false, true, 400, false, false, 2),
	(97, 9911, 991106, 'VIVIENDA, ACABADOS DE CASA', 'vivienda', false, true, 400, false, false, 2),
	(98, 9911, 991107, 'VIVIENDA, INSTALACIONES HIDRAÚLICAS Y SANITARIAS', 'vivienda', false, true, 400, false, false, 2),
	(99, 9911, 991108, 'VIVIENDA, INSTALACIÓN ELÉCTRICA', 'vivienda', false, true, 400, false, false, 2),
	(100, 9911, 991109, 'VIVIENDA, CARPINTERÍA ', 'vivienda', false, true, 400, false, false, 2),
	(101, 9911, 991110, 'VIVIENDA, CANCELERÍA,  HERRERÍA Y CRISTALES', 'vivienda', false, true, 400, false, false, 2),
	(102, 9911, 991111, 'VIVIENDA, JARDINERÍA ', 'vivienda', false, true, 0, false, false, 2),
	(103, 9911, 991112, 'VIVIENDA, AMPLIACION DE VIVIENDA', 'vivienda', false, true, 0, false, false, 2),
	(104, 9911, 991113, 'VIVIENDA, MEJORAMIENTO Y REMODELACIÓN DE CASA', 'vivienda', false, true, 400, false, false, 2),
	(105, 9911, 991114, 'VIVIENDA, ENGANCHE DE CASA', 'vivienda', false, true, 400, false, false, 2),
	(106, 9811, 981101, 'GASTOS DE TEMPORADA', 'consumo', false, true, 0, false, false, 3),
	(107, 9811, 981102, 'PAGO DE DEUDAS', 'consumo', true, true, 0, false, false, 3),
	(108, 9811, 981103, 'GASTOS DE ALIMENTACIÓN ', 'consumo', true, true, 0, false, false, 3),
	(109, 9811, 981104, 'GASTOS ESCOLARES', 'consumo', true, true, 0, false, false, 1),
	(110, 9811, 981105, 'GASTOS DE SEGUROS', 'consumo', true, true, 0, false, false, 3),
	(111, 9811, 981106, 'GASTOS DE RECREACIÓN', 'consumo', false, true, 0, false, false, 3),
	(112, 9811, 981107, 'GASTOS DE IMPUESTOS', 'consumo', true, true, 0, false, false, 3),
	(113, 9811, 981108, 'GASTOS PERSONALES ESPECíFICOS', 'consumo', false, true, 0, false, false, 2),
	(114, 9811, 981109, 'GASTOS PERSONALES VARIOS', 'consumo', false, true, 0, false, false, 2),
	(115, 9811, 981110, 'COMPRA DE ARTICULOS PARA EL HOGAR', 'consumo', false, true, 0, false, false, 3),
	(116, 1110, 1110101, 'REFACCIONARIO AGRICOLA, ACONDICIONAMIENTO Y MEJORAS DE TERRENOS DE CULTIVO.', 'comercio', false, true, 100, true, false, 3),
	(117, 1110, 1110201, 'REFACCIONARIO, ADQUISICIÓN DE VEHÍCULOS PARA USO PRODUCTIVO', 'comercio', false, true, 100, false, false, 3),
	(118, 1110, 1110202, 'REFACCIONARIO AGRICOLA, ADQUISICIÓN O MANTENIMIENTO DE SISTEMAS DE RIEGO', 'comercio', false, true, 100, false, false, 3),
	(119, 1110, 1110203, 'REFACCIONARIO AGRICOLA, INSTALACIÓN O MANTENIMIENTO DE POZOS PARA RIEGO', 'comercio', false, true, 100, false, false, 3),
	(120, 1110, 1110204, 'EXPLOTACIÓN FORESTAL', 'comercio', false, false, 100, true, false, 3),
	(121, 1110, 1110205, 'AVIO AGRICOLA, CULTIVO DE AGUACATE', 'comercio', false, false, 100, true, false, 3),
	(122, 1110, 1110206, 'AVIO AGRICOLA, CULTIVO DE APIO', 'comercio', false, false, 100, true, false, 3),
	(123, 1110, 1110207, 'AVIO AGRICOLA, CULTIVO DE FRESA', 'comercio', false, false, 100, true, false, 3),
	(124, 1110, 1110208, 'AVIO AGRICOLA, CULTIVO DE BROCOLI', 'comercio', false, false, 100, true, false, 3),
	(125, 1110, 1110209, 'AVIO AGRICOLA, CULTIVO DE SORGO', 'comercio', false, false, 100, true, false, 3),
	(126, 1110, 1110210, 'AVIO AGRICOLA, CULTIVO DE CEBADA', 'comercio', false, false, 100, true, false, 3),
	(127, 1110, 1110211, 'AVIO AGRICOLA, CULTIVO DE FRIJOL', 'comercio', false, false, 100, true, false, 3),
	(128, 1110, 1110212, 'AVIO AGRICOLA, CULTIVO DE AVENA', 'comercio', false, false, 100, true, false, 3),
	(129, 1110, 1110213, 'AVIO AGRICOLA, CULTIVO DE GUAYABA', 'comercio', false, false, 100, true, false, 3),
	(130, 1110, 1110214, 'AVIO AGRICOLA, CULTIVO DE NOCHEBUENA', 'comercio', false, false, 100, true, false, 3),
	(131, 1110, 1110215, 'AVIO AGRICOLA, CULTIVO DE CHAYOTE', 'comercio', false, false, 100, true, false, 3),
	(132, 1124, 1124101, 'AVIO GANADERO, CHIVAS', 'comercio', false, false, 100, false, true, 3),
	(133, 1124, 1124102, 'AVIO GANADERO, CONEJOS', 'comercio', false, false, 100, false, true, 3),
	(134, 1124, 1124103, 'AVIO GANADERO, GALLINAS PONEDORAS', 'comercio', false, false, 100, false, true, 3),
	(135, 1125, 1125101, 'REFACCIONARIO GANADERÍA, SEMENTALES Y PIES DE CRIA', 'comercio', false, true, 100, false, false, 3),
	(136, 3323, 3323201, 'FABRICACIÓN DE PRODUCTOS DE CARPINTERÍA', 'comercio', false, true, 200, false, false, 3),
	(137, 3323, 3323202, 'FABRICACIÓN DE ARTESANÍAS', 'comercio', false, true, 200, false, false, 3),
	(138, 3323, 3323203, 'FABRICACION DE PRODUCTOS LÁCTEOS (QUESOS, CREMAS, ETC.)', 'comercio', false, true, 227, false, false, 3),
	(139, 3323, 3323204, 'FABRICACION DE EMBUTIDOS (JAMÓN, SALCHICHA, ETC.)', 'comercio', false, true, 228, false, false, 3),
	(140, 3323, 3323205, 'FABRICACION DE ALIMENTOS PARA EL GANADO', 'comercio', false, true, 200, false, false, 3),
	(141, 3323, 3323206, 'REFACCIONARIO PARA EMPRESAS INDUSTRIALES', 'comercio', false, true, 200, false, false, 3),
	(142, 4684, 4684201, 'COMERCIO, PRODUCTOS DIVERSOS MAYOREO', 'comercio', false, true, 300, false, false, 3),
	(143, 4684, 4684202, 'COMERCIO, OTRAS EMPRESAS DE COMERCIO', 'comercio', false, true, 300, false, false, 3),
	(144, 4684, 4684203, 'REFACCIONARIO PARA COMERCIANTES', 'comercio', false, true, 300, false, false, 3),
	(145, 7211, 72111101, 'SERVICIOS, OTRAS EMPRESAS DE SERVICIOS', 'comercio', false, true, 400, false, false, 3),
	(146, 7211, 72111102, 'REFACCIONARIO PARA EMPRESAS DE SERVICIOS', 'comercio', false, true, 400, false, false, 3),
	(147, 9811, 98110101, 'PAGO DE HIPOTECAS', 'consumo', false, true, 0, false, false, 3),
	(148, 9911, 99110102, 'VIVIENDA, COMPRA DE CASA NUEVA O USADA', 'vivienda', false, true, 400, false, false, 2),
	(149, 9911, 99110103, 'VIVIENDA, COMPRA DE TERRENO CON SERVICIOS', 'vivienda', false, true, 400, false, false, 2),
	(156, 9811, 98110108, 'GASTOS POR MANTENIMIENTO O REPARACION DE BIENES DE CONSUMO DURADERO', 'consumo', false, true, 0, false, false, 3),
	(151, 9811, 98110102, 'GASTOS MEDICOS', 'consumo', false, true, 0, false, false, 1),
	(153, 9811, 98110105, 'GASTOS DE MANTENIMIENTO Y REPARACION DE VEHICULO USO PERSONAL', 'consumo', false, true, 0, false, false, 2),
	(154, 9811, 98110106, 'COMPRA DE MUEBLES LINEA BLANCA Y ELECTRONICA', 'consumo', false, true, 0, false, false, 3),
	(155, 9811, 98110107, 'COMPRA DE VEHICULO DE USO PERSONAL', 'consumo', false, true, 0, false, false, 2),
	(157, 9811, 98110109, 'GASTOS POR EVENTOS SOCIALES', 'consumo', false, true, 0, false, false, 3),
	(158, 9811, 98110110, 'PAGO DE SERVICIOS PUBLICOS', 'consumo', false, true, 0, false, false, 3),
	(159, 9811, 98110111, 'GASTOS FUNERARIOS', 'consumo', false, true, 0, false, false, 3),
	(160, 9811, 98110112, 'GASTOS DE VIAJE', 'consumo', false, true, 0, false, false, 3),
	(161, 4611, 4611201, 'COMERCIO, COMPRA VENTA DE POLLO EN PLUMA Y PROCESADO', 'comercio', false, false, 400, false, false, 3),
	(163, 9911, 991115, 'COMPRA E INSTALACION DE CALENTADOR SOLAR PARA NEGOCIO', 'comercio', false, true, 0, false, false, 3),
	(164, 9911, 991116, 'COMPRA E INSTALACION DE SISTEMAS FOTOVOLTAICOS PARA NEGOCIO', 'comercio', false, true, 0, false, false, 3),
	(169, 9911, 991117, 'COMPRA E INSTALACION DE CALENTADOR SOLAR PARA VIVIENDA', 'consumo', false, true, 0, false, false, 2),
	(170, 9911, 991118, 'COMPRA E INSTALACION DE SISTEMAS FOTOVOLTAICOS PARA VIVIENDA', 'consumo', false, true, 0, false, false, 2),
	(173, 3119, 31200, 'FABRICACION DE PRODUCTOS DE HERRERIA', 'comercio', false, true, 200, false, false, 3);


