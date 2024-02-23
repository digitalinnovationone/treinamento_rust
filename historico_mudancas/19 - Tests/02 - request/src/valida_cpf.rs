pub fn valida_cpf(cpf: &str) -> bool {
    let cpf_numeros: Vec<u32> = cpf.chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    if cpf_numeros.len() != 11 || cpf_numeros.iter().all(|&n| n == cpf_numeros[0]) {
        return false;
    }

    let mut soma = 0;
    for i in 0..9 {
        soma += cpf_numeros[i] * (10 - i as u32);
    }
    let resto = (soma * 10) % 11;
    let primeiro_digito = if resto == 10 { 0 } else { resto };

    soma = 0;
    for i in 0..10 {
        soma += cpf_numeros[i] * (11 - i as u32);
    }
    let resto = (soma * 10) % 11;
    let segundo_digito = if resto == 10 { 0 } else { resto };

    primeiro_digito == cpf_numeros[9] && segundo_digito == cpf_numeros[10]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_cpf_valido() {
        assert!(valida_cpf("52998224725")); // Um CPF válido
    }

    #[test]
    fn testa_cpf_valido_formatado() {
        assert!(valida_cpf("708.023.730-03")); // Um CPF válido
    }

    #[test]
    fn testa_cpf_invalido() {
        assert!(!valida_cpf("11111111111")); // Um CPF inválido (dígitos iguais)
    }

    #[test]
    fn testa_cpf_invalido_formatado() {
        assert!(!valida_cpf("708.023.730-01"));
    }

    #[test]
    fn testa_formato_invalido() {
        assert!(!valida_cpf("123")); // Formato inválido
    }

    #[test]
    fn valida_cpf_corretamente() {
        // Arrange
        let cpf_valido = "529.982.247-25";
        let cpf_invalido = "111.111.111-11";

        // Act
        let resultado_valido = valida_cpf(cpf_valido);
        let resultado_invalido = valida_cpf(cpf_invalido);

        // Assert
        assert!(resultado_valido, "O CPF deveria ser considerado válido");
        assert!(!resultado_invalido, "O CPF deveria ser considerado inválido");
    }




    //// ===== Outros TODO buscar mais ========

    #[test]
    fn testa_retorno_funcao() {
        let valor_esperado = 42; 
        let resultado = 42;
        assert_eq!(resultado, valor_esperado, "A função não retornou o valor esperado!");
    }
}
