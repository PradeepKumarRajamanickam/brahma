extern crate codegen;
use bevy::{AddDefaultPlugins, prelude::App, window::{self, WindowDescriptor}};
use brahma::BrahmaPlugin;
use codegen::Scope;

fn main()
{
       App::build()
        .add_resource(WindowDescriptor { // <--
            title: "brahma".to_string(), // <--
            width: 500,                 // <--
            height: 500,                // <--
            ..Default::default()         // <--
        })
        .add_default_plugins()
        .add_plugin(BrahmaPlugin::default())
        .run();
}

// fn main() {
    
//     // let mut scope = Scope::new();

//     // scope
//     //     .new_struct("Foo")
//     //     // .derive("Debug")
//     //     .field("one", "usize")
//     //     .field("two", "String");

//     // scope
//     //     .new_fn("add");
        

//     // // scope
//     // //     .im

//     // println!("CODE \n{}\n\n AST", scope.to_string());
//     let code = "
// pub struct InputMap;
// impl InputMap {
//     pub const MAX_PLAYER_HANDLES: usize = 8;
// }
// ";

//     // let code = "assert_eq!(u8::max_value(), 255)";
//     let ast = syn::parse_file(code).unwrap();
//     // let v = ast.items.get_mut(0).unwrap();
//     println!("{:#?}", ast);
// }
