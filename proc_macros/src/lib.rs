use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn bevy_app(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let name = &input_fn.sig.ident;
    let expanded = quote! {
        struct BevyExtensionLibrary;

        #[gdextension]
        unsafe impl ExtensionLibrary for BevyExtensionLibrary {
            // fn load_library(handle: &mut InitHandle) -> bool {
            //     handle.register_layer(InitLevel::Scene, InitializationLayer);
            //     true
            // }

            fn on_level_init(level: bevy_godot4::prelude::InitLevel) {
                bevy_godot4::godot::private::class_macros::auto_register_classes(level);

                let mut app_builder_func = bevy_godot4::APP_BUILDER_FN.lock().unwrap();
                if app_builder_func.is_none() {
                    *app_builder_func = Some(Box::new(#name));
                }
            }

        }

        // pub struct InitializationLayer;

        // impl ExtensionLayer for InitializationLayer {
        //     fn initialize(&mut self) {
        //         bevy_godot4::godot::private::class_macros::auto_register_classes();

        //         let mut app_builder_func = bevy_godot4::APP_BUILDER_FN.lock().unwrap();
        //         if app_builder_func.is_none() {
        //             *app_builder_func = Some(Box::new(#name));
        //         }
        //     }

        //     fn deinitialize(&mut self) {}
        // }

        #input_fn

    };

    expanded.into()
}
