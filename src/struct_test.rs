// struct definiton
#[derive(Debug)]
pub struct StructName 
{
	pub name: String,
	pub address: String,
	pub age: u32,
}


pub fn take_struct_input(mut param_name:StructName) -> StructName
{
	param_name.age = param_name.age + 2;
	println!("{:#?}", param_name);
	return param_name;
}