# RS PERMISSIONS

Es un pequeño proyecto creado para calcular BitPermissions a partir de una lista en un JSON en el siguiente formato:

```json
{
    "permissions": [
        // [Nombre, Valor]
        ["Read Database", 1]
    ]
}
```

El CLI debe usarse de esta manera:

```bash
./ruta/al/binario/r-perms --dir=ruta/al/archivo/permissions.json
```

Verá una salida como esta:

```bash
Permissions loaded from JSON:
[1] Uno
[4] Dos
? Select the permissions ›
✔ Uno
✔ Dos
```

Luego de seleccionar los permisos correspondientes, la consola será limpiada y verá una salida como esta:

```bash
Calculated Permissions Value: 5
```
