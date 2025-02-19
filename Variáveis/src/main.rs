//! # Os tipos s'ao seus amigos mano
//! 
//! Rust [e uma linguagem de programa;'ao com tipagem estatica e forte, o que significa que:
//! - Tipagem estática: O tipo de cada variável é determinado em tempo de compilação e não muda.
//! Uma variável declarada com um tipo específico não pode ser restribuída a um valor de outro tipo sem uma conversão e
//! Tipagem forte: Rust é rigoroso e uma string sem converter explicitamente um deles para o tipo compatível o outro.
//! 
//! O compilador do Rust é inteligente o suficiente para inferir o tipo de muitas variáveis.
//! O que significa que nem sempre é necessário especificar o tipo explicitamente.
//! 
//! Nest aula, focaremos nos tipos primitivos, que são os blocos de construção básicos para a criação de programas em



fn main() {

    println!("{}", std::mem::size_of::<i8>());
    println!("{}", std::mem::size_of::<(i32, i8, i8)>());


}