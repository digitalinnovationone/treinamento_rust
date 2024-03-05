pub fn divide(numerator: i32, denominator: i32) -> Result<i32, String> {
    if denominator == 0 {
        Err(String::from("Erro: Divisão por zero."))
    } else {
        Ok(numerator / denominator)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testa_retorno_funcao() {
        let valor_esperado = 42; 
        let resultado = 42;
        assert_eq!(resultado, valor_esperado, "A função não retornou o valor esperado!");
    }


    #[test]
    #[ignore = "Exemplo de como ignorar um test"]
    fn teste_ignorado() {
        println!("....")
    }


    #[test]
    fn divide_sucesso() -> Result<(), String> {
        let result = divide(10, 2)?;
        assert_eq!(result, 5);
        Ok(())
    }

    #[test]
    fn divide_erro() -> Result<(), String> {
        let result = divide(10, 0);
        assert!(result.is_err());
        assert_eq!(result, Err(String::from("Erro: Divisão por zero.")));
        Ok(())
    }
}
