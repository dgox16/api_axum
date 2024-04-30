# API en AXUM

Proyecto backend desarrollado con el framework de [Axum](https://github.com/tokio-rs/axum) con la intenciín de generar una API usando el lenguaje de Rust.

## Guia de instalación

En esta guia se da por hecho que tienes PostgreSQL ya instalado y completamente funcional. Lo primero que debemos hacer es clonar el repositorio:

```bash
git clone https://github.com/dgox16/api_axum.git
```

Despues de eso crearemos un archivo .env donde se encontraran algunas configuraciones como el url de la base de datos, una palabra secreta para el uso de JWT e indicar el tiempo de expiraciín del token. Para conseguir esto primeramente realizamos lo siguiente:

```bash
cd api_axum
touch .env
```

Y en ese archivo crearemos las siguientes variables globales (recuerda quitar los simbolos de < >):

```
DATABASE_URL=<AQUI COLOCA LA URL A TU BASE DE DATOS>
JWT_SECRETO=<AQUI COLOCA UNA PALABRA SECRETA>
JWT_EXPIRA_EN=<AQUI COLOCA UN NUMERO QUE SERAN EL TIEMPO EN MINUTOS (EJEMPLO: 300)>
```

Con esto ya podemos empezar a configurar la base de datos. Para usar la base de datos usaremos la herramienta de [sqlx](https://github.com/launchbadge/sqlx); para ello lo instalaremos con el siguiente comando:

```bash
cargo install sqlx-cli
```

En caso de no haber creado la base de datos, podemos usar esta herramienta para crearla automaticamente porque toma la variable creada en el .env. Para conseguir esto usamos el comando de `sqlx database create`. Con la base de datos creada ejecutaremos las migraciones para crear las tablas necesarias:

```bash
sqlx migrate run
```

Finalmente, solamente ejecutamos el comando para correr el proyecto con el uso de cargo:

```bash
cargo run
```
