use std::io;
mod verifica_entrada;
mod gbe;
mod constantes;

fn pergunta() -> String {
	println!("Digite: ");
	let mut entrada: String = String::new();
    io::stdin().read_line(&mut entrada).expect("Falha ao ler a entrada");
	entrada
}

fn main() {
	let pergunta: String = pergunta();
    let verificacao: verifica_entrada::Conjunto = verifica_entrada::verifica_retorno(pergunta);

	let gbe: f64 = gbe::gbe(verificacao.massa, verificacao.raio, constantes::G);
	println!("{}", gbe / constantes::FATOR_CONVERSAO);
}
