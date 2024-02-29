Instalar o disel
```shell
cargo install diesel_cli --no-default-features --features mysql
```

Disel setup
```shell
diesel setup

```
Gerar as migrations
```shell
diesel migration generate create_clientes
```


Rodar a migration
```shell
diesel migration run
```

Voltar a migration
```shell
diesel migration revert
```

