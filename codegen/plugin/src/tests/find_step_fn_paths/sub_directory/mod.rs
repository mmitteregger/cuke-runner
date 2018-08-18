pub mod public_nested_module;
mod private_module_in_directory;

pub fn sub_directory_fn() {}

fn private_sub_directory_fn() {}

mod private_inner_module {
    fn private_function_in_private_module() {}

    pub fn public_function_in_private_module() {}
}

pub mod public_inner_module {
    fn private_function_in_public_module() {}
    pub fn public_function_in_public_module() {}

    pub mod public_nested_inner_module {
        pub fn public_function_in_public_nested_inner_module() {}
    }
}
