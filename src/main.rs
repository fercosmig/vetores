fn main()
{
    {
        // forma tradicional
        let mut meu_vetor = vec![10, 20, 30, 40, 50];
        println!("A:{:?}", meu_vetor);

        // adicionando item no final do vetor
        meu_vetor.push(60);
        println!("B:{:?}", meu_vetor);

        // removendo item do vetor pelo indice
        meu_vetor.remove(5);
        println!("C:{:?}", meu_vetor);

        // imprimindo o vetor inteiro
        println!("D:{:?}", meu_vetor);

        // imprimindo com for
        for item in meu_vetor.iter()
        {
            println!("E:{}", item);
        }
    }

    {
        // instanciando um vetor de inteiros já alocando espaço na memória
        let meu_vetor: Vec<i32> = Vec::new();
    }
}
