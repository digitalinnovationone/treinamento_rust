macro_rules! cria_struct {
    (
        $nome_struct:ident {
            $($campo:ident: $tipo:ty),* $(,)?
        }
        $(fn $nome_metodo:ident(&$nome_metodo_struct:ident $(, $param_nome:ident: $param_tipo:ty)*) -> $ret_tipo:ty $corpo_metodo:block)*
    ) => {
        struct $nome_struct {
            $($campo: $tipo,)*
        }

        impl $nome_struct {
            $(
                fn $nome_metodo(&$nome_metodo_struct $(, $param_nome: $param_tipo)*) -> $ret_tipo $corpo_metodo
            )*
        }
    };
}