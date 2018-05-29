pub mod motto_motto;
mod priv_mod_in_dir;

pub fn something_here() {}

fn priv_here_too() {}

mod priv_inner_mod {
    fn foo()  {}
    pub fn bar() {}
}

pub mod pub_inner_mod_1 {
    fn foo() {}
    pub fn bar() {}

    pub mod pub_inner_mod_2 {
        pub fn how_far_can_we_go() {}
    }
}
