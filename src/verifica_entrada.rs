pub struct Conjunto {
    pub massa: f64,
    pub raio: f64,
}

pub fn verifica_escala(tipo:u8) -> Conjunto {

    match tipo {
        1 =>  Conjunto { massa : 2.3 * 10_f64.powi(20), raio : 2.0 * 10_f64.powi(6) } ,
        2 =>  Conjunto { massa : 2.6 * 10_f64.powi(26), raio : 5.0 * 10_f64.powi(7) } ,
        _ =>  Conjunto { massa: 0.0, raio: 0.0 }
    }
}

// Retornará um conjunto que é atribuído a classe de destruição inserida.
pub fn verifica_retorno(nome: String) -> Conjunto {
	match nome {
        x if 
            x.contains("lanet") && 
            (x.contains("equeno") | 
            x.contains("mall")) 
       => verifica_escala(1),
       x if 
            x.contains("lanet") && 
            (x.contains("rande") | 
            x.contains("ig"))
        => verifica_escala(2),
        _ => Conjunto { massa: 0.0, raio: 0.0 }
    }
}