//-------------------------------------------------------------------------------------------------
//         Zero Sized Types
//--------------------------------------------------------------------------------------------------

pub mod never_type;
pub mod unit_type;
pub mod unit_struct;

pub fn main() {
    println!();
    println!("========= Never Type =========");
    never_type::main();

    println!();
    println!("========= Unit Type =========");
    unit_type::main();
    
    println!();
    println!("========= Unit Struct =========");
    unit_struct::main();
}