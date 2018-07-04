#![allow(non_snake_case)]

use project;

#[test]
fn apply() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn apply(target: &js::Function, this_argument: &JsValue, arguments_list: &js::Array) -> JsValue {
                let result = js::Reflect::apply(target, this_argument, arguments_list);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                assert.equal(wasm.apply("".charAt, "ponies", [3]), "i");
                assert.equal(wasm.apply("", "ponies", [3]), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn construct() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn construct(target: &js::Function, arguments_list: &js::Array) -> JsValue {
                let result = js::Reflect::construct(target, arguments_list);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                class Rectangle {
                    public x: number;
                    public y: number;

                    constructor(x: number, y: number){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x: number, y: number) { 
                        return x === y;
                    }

                }

                const args = [10, 10];

                assert.equal(wasm.construct(Rectangle, args).x, 10);
                assert.equal(wasm.construct("", args), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn construct_with_new_target() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn construct_with_new_target(target: &js::Function, arguments_list: &js::Array, new_target: &js::Function) -> JsValue {
                let result = js::Reflect::construct_with_new_target(target, arguments_list, new_target);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                class Rectangle {
                    public x: number;
                    public y: number;

                    constructor(x: number, y: number){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x: number, y: number) { 
                        return x === y;
                    }

                }

                class Rectangle2 {
                    public x: number;
                    public y: number;

                    constructor(x: number, y: number){
                        this.x = x,
                        this.y = y
                    }

                    static eq(x: number, y: number) { 
                        return x === y;
                    }

                }

                const args = [10, 10];

                assert.equal(wasm.construct_with_new_target(Rectangle, args, Rectangle2).x, 10);
                assert.equal(wasm.construct_with_new_target(Rectangle, args, ""), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn define_property() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn define_property(target: &js::Object, property_key: &JsValue, attributes: &js::Object) -> JsValue {
                let result = js::Reflect::define_property(target, property_key, attributes);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {};

                assert.equal(wasm.define_property(object, "key", { value: 42}), true)
                assert.equal(wasm.define_property("", "key", { value: 42 }), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn delete_property() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn delete_property(target: &JsValue, property_key: &JsValue) -> JsValue {
                let result = js::Reflect::delete_property(target, property_key);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                wasm.delete_property(object, "property");

                assert.equal(object.property, undefined);

                const array = [1, 2, 3, 4, 5];
                wasm.delete_property(array, 3);

                assert.equal(array[3], undefined);

                assert.equal(wasm.delete_property("", 3), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn get() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get(target: &JsValue, property_key: &JsValue) -> JsValue {
                let result = js::Reflect::get(target, property_key);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                assert.equal(wasm.get(object, "property"), 42);

                const array = [1, 2, 3, 4, 5];

                assert.equal(wasm.get(array, 3), 4);

                assert.equal(wasm.get("", 3), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn get_own_property_descriptor() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_own_property_descriptor(target: &JsValue, property_key: &JsValue) -> JsValue {
                let result = js::Reflect::get_own_property_descriptor(target, property_key);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                assert.equal(wasm.get_own_property_descriptor(object, "property").value, 42);
                assert.equal(wasm.get_own_property_descriptor(object, "property1"), undefined);
                assert.equal(wasm.get_own_property_descriptor("", "property1"), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn get_prototype_of() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn get_prototype_of(target: &JsValue) -> JsValue {
                let result = js::Reflect::get_prototype_of(target);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };
                const array: number[] = [1, 2, 3];

                assert.equal(wasm.get_prototype_of(object), Object.prototype);
                assert.equal(wasm.get_prototype_of(array), Array.prototype);
                assert.equal(wasm.get_prototype_of(""), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn has() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn has(target: &JsValue, property_key: &JsValue) -> JsValue {
                let result = js::Reflect::has(target, property_key);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };
                const array: number[] = [1, 2, 3, 4]

                assert.equal(wasm.has(object, "property"), true);
                assert.equal(wasm.has(object, "foo"), false);
                assert.equal(wasm.has(array, 3), true);
                assert.equal(wasm.has(array, 10), false);
                assert.equal(wasm.has("", "property"), "TypeError");
            }
        "#,
        )
        .test()
}

#[test]
fn is_extensible() {
    project()
        .file(
            "src/lib.rs",
            r#"
            #![feature(proc_macro, wasm_custom_section)]

            extern crate wasm_bindgen;
            use wasm_bindgen::prelude::*;
            use wasm_bindgen::js;

            #[wasm_bindgen]
            pub fn is_extensible(target: &js::Object) -> JsValue {
                let result = js::Reflect::is_extensible(target);
                let result = match result {
                    Ok(val) => val,
                    Err(_err) => "TypeError".into()
                };
                result
            }
        "#,
        )
        .file(
            "test.ts",
            r#"
            import * as assert from "assert";
            import * as wasm from "./out";

            export function test() {
                const object = {
                    property: 42
                };

                assert.equal(wasm.is_extensible(object), true);

                Reflect.preventExtensions(object);

                assert.equal(wasm.is_extensible(object), false);

                const object2 = Object.seal({});

                assert.equal(wasm.is_extensible(object2), false);
                assert.equal(wasm.is_extensible(""), "TypeError");
            }
        "#,
        )
        .test()
}