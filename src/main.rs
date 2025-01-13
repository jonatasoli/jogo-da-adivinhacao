use rand::{thread_rng, Rng};
use std::io;

#[derive(Debug, PartialEq)]
enum GameResult {
    Win,
    Gaming,
    Lose,
}

fn main() {
    loop {
        println!("Bem vindo ao jogo da adivinhação escolha uma das opções abaixo");
        println!("i - Iniciar o jogo");
        println!("q - Fechar o jogo");

        let mut escolha_str = String::new();
        io::stdin()
            .read_line(&mut escolha_str)
            .expect("Erro ao receber sua escolha");

        match escolha_str.trim().to_lowercase().as_str() {
            "i" => {
                game();
                continue;
            }
            "q" => {
                println!("Obrigado por jogar");
                break;
            }
            _ => {
                println!("Escolha inválida. Tente novamente.");
                continue;
            }
        };
    }
}

fn game() {
    let mut pontuacao: u16 = 1000;
    let numero_alvo: u8 = thread_rng().gen_range(1..100);
    loop {
        println!("Por favor digite o número que você acredita ser");
        let mut chute = String::new();
        io::stdin()
            .read_line(&mut chute)
            .expect("Erro ao receber o número");

        let chute: u8 = match chute.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Valor não é válido ou não está entre 0 e 255");
                0
            }
        };
        match check_win_coditition(&mut pontuacao, &numero_alvo, &chute) {
            Ok(result) => {
                if result == GameResult::Win {
                    println!("Parabéns você venceu! Sua pontuação foi {}", pontuacao);
                    break;
                } else if result == GameResult::Lose {
                    println!("Que pena você perdeu!");
                    break;
                }
            }
            Err(_) => {
                println!("Ocorreu um erro e o jogo será reiniciado!");
                break;
            }
        };
        println!("A sua pontuação está em {}", pontuacao);
    }
}

fn check_win_coditition(
    pontuacao: &mut u16,
    numero: &u8,
    chute: &u8,
) -> Result<GameResult, GameResult> {
    if chute < numero {
        println!("O número é maior!");
        *pontuacao -= 100;
    }
    if chute > numero {
        println!("O número é menor!");
        *pontuacao -= 100;
    }
    if chute == numero {
        return Ok(GameResult::Win);
    }
    if *pontuacao == 0 {
        return Ok(GameResult::Lose);
    }
    Ok(GameResult::Gaming)
}

#[test]
fn test_jogador_deu_numero_errado_pra_baixo_deve_diminuir_pontuacao_geral() {
    // Arrange
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 1;

    // Act
    let result = check_win_coditition(&mut pontuacao, &numero, &chute);

    // Assert
    assert_eq!(result, Ok(GameResult::Gaming));
    assert_eq!(pontuacao, 900)
}

#[test]
fn test_jogador_deu_numero_errado_pra_cima_deve_diminuir_pontuacao_geral() {
    // Arrange
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 100;

    // Act
    let result = check_win_coditition(&mut pontuacao, &numero, &chute);

    // Assert
    assert_eq!(result, Ok(GameResult::Gaming));
    assert_eq!(pontuacao, 900)
}

#[test]
fn test_jogador_deu_numero_exato_deve_finalizar_jogo_sem_mudar_pontuacao() {
    // Arrange
    let mut pontuacao: u16 = 1000;
    let numero: u8 = 42;
    let chute: u8 = 42;

    //Act
    let result = check_win_coditition(&mut pontuacao, &numero, &chute);

    //Asert
    assert_eq!(result, Ok(GameResult::Win));
    assert_eq!(pontuacao, 1000)
}

#[test]
fn test_jogador_deu_numero_errado_baixo_deve_finalizar_jogo_perdendo() {
    // Arrange
    let mut pontuacao: u16 = 100;
    let numero: u8 = 42;
    let chute: u8 = 1;

    //Act
    let result = check_win_coditition(&mut pontuacao, &numero, &chute);

    //Asert
    assert_eq!(result, Ok(GameResult::Lose));
    assert_eq!(pontuacao, 0)
}

#[test]
fn test_jogador_deu_numero_errado_alto_deve_finalizar_jogo_perdendo() {
    // Arrange
    let mut pontuacao: u16 = 100;
    let numero: u8 = 42;
    let chute: u8 = 100;

    //Act
    let result = check_win_coditition(&mut pontuacao, &numero, &chute);

    //Asert
    assert_eq!(result, Ok(GameResult::Lose));
    assert_eq!(pontuacao, 0)
}
