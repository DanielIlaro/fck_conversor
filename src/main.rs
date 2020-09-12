// O fck_conversor, faz conversões de temperatura entre as escalas Celsius, Fahrenhreit e Kelvin.

use std::io;

/* 
*   Essa função faz o cálculo da temperatura:
*       - opc é do tipo u32 e recebe o valor da opção escolhida pelo usuario
*       - valor é do tipo f64 e recebe o valor que deve ser convertido para a outra escala
*/

fn calcular_temperatura(opc: u32, valor : f64) -> String{
    // 0 K em Graus celsius
    const C : f64 = -273.15;

    // 0 °C em Fahrenhreit
    const F : f64 = 32.0;


    match opc {
        1 => {
                let res = valor + C;
                res.to_string()+" °C"
        }

        2 => {
            let res = (valor + C) * 1.8 + F;
            res.to_string()+" °F"
        }
        3 => {
            let res = valor - C;
            res.to_string()+" K"
        }
        4 => {
            let res = valor * 1.8 + F;
            res.to_string()+" °F"
        }
        5 => {
            let res = (valor - F) / 1.8 - C;
            res.to_string()+" K"
        }
        
        6 => {
            let res = (valor - F) / 1.8;
            res.to_string()+" °C"
        }
        _ => "Erro".to_string()
    }
}

fn main() {
    loop{
        let mut valor = String::new();
        let mut escolha = String::new();

        println!("Escolha uma das opções:\n
                1- Kelvin para Graus Celsius\n
                2- Kelvin para Graus Fahrenhreit\n
                3- Graus Celsius para Kelvin\n
                4- Graus Celsius para Graus Fahrenhreit\n
                5- Graus Fahrenhreit para Kelvin\n
                6- Graus Fahrenhreit para Graus Celsius\n
                7- Sair\n");

        println!("Digite a escolha: ");
        io::stdin().read_line(&mut escolha)
            .expect("Erro ao escolher");

        let escolha : u32 = match escolha.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                        println!("Erro ao converter\nVai ser utilizado o padrão Kelvin para Graus Celsius.");
                1
            }
        };

        if escolha == 7{
            break
        }

        else if escolha > 7 && escolha < 1{
            println!("A opção {} não é valida", escolha);
            continue
        }

        println!("Digite o número que deseja converter: ");
        io::stdin().read_line(&mut valor)
            .expect("Erro ao passar o valor");

        let valor = match valor.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("Erro ao converter para Float, vai ser passado o valor 0.0");
                0.0
            }
        };

        let resultado = calcular_temperatura(escolha, valor);

        println!("Resultado: {}\n
==========================================\n", resultado);

        println!("Continuar convertendo? S/N?");
        let mut sorn = String::new(); 

        io::stdin().read_line(&mut sorn)
            .expect("Erro ao passar o valor");

        if sorn.trim().to_lowercase().eq("n"){
            break
        }
    }
}