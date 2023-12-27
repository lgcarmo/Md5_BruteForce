use md5;
use std::io::{self, BufRead};
use std::env::args;
use std::fs::File;

fn main() -> std::io::Result<()>{

    //coleta os argumentos e armazena em uma colecao ARGS
    let args: Vec<String> = args().collect();

    if args.len() < 3 {
        println!("Uso: {} <target_md5> <nome_do_arquivo>", args[0]);
        return Ok(());
    }
    
    // realiza a leitura do arquivo informado no primeiro argumento
    let file = File::open(&args[2])?;

    //realiza o buffer do arquivo
    let reader = io::BufReader::new(file);

    let target_md5 = &args[1];


    //loop das linhas do arquivo
    for line in reader.lines(){
        
        //valor da linha em string
        let line_str = match line {
            Ok(val) => val,
            Err(_) => {
                continue;
            }
        };

        //computacao da hash no formato de bytes
        let computed_md5 = format!("{:x}", md5::compute(line_str.as_bytes()));

        //println!("{}: {:x}", line_str, digest);

        if computed_md5 == target_md5.to_string(){
            println!("[+] PasswordFound : {} Hash: {}", line_str, computed_md5);
            break;
        }
    }

    Ok(())
}
