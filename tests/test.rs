#[cfg(test)]
mod tests {
	
	use math_lib_test::{add, sub, take_retrun_string};
	use math_lib_test::mul_div::{mul,div};
	use assert_str::assert_str_eq;


	use math_lib_test::struct_test::{StructName, take_struct_input};

    #[test]
    fn it_works1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

		#[test]
		fn it_works2() {
	
			let result = sub(4,1);
			assert_eq!(result,3);
	}

	#[test]
		fn it_works3() {
	
			let result = mul(4,2);
			assert_eq!(result,8);
	}

	#[test]
		fn it_works4() {
	
			let result = div(4,2);
			assert_eq!(result,2);
	}

	#[test]
		fn it_works5() {
			let s = "hello";
			let result = take_retrun_string(s.to_string());
			assert_str_eq!(result,s);
	}



	#[test]
		fn it_works6() {
			let user1 = StructName {
    	name: String::from("Mahmudul"),
    	address: "dhaka".to_string(),
    age: 24,
		};

		let result = take_struct_input(user1);
		assert_str_eq!(result.name,"Mahmudul");
		assert_str_eq!(result.address,"dhaka");
		assert_eq!(result.age,26);


	}
}