use web_sys::HtmlInputElement;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let encode_input_node_ref = use_node_ref();

    let encode_input_value_handle = use_state(String::default);
    let encode_input_value = (*encode_input_value_handle).clone();

    let encode_input_onchange = {
        let encode_input_node_ref = encode_input_node_ref.clone();
        Callback::from(move |_| {
            let input = encode_input_node_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                encode_input_value_handle.set(input.value());
            }
        })
    };

    let encode_shift_node_ref = use_node_ref();

    let encode_shift_value_handle = use_state(String::default);
    let encode_shift_value = (*encode_shift_value_handle).clone();

    let encode_shift_onchange = {
        let encode_shift_node_ref = encode_shift_node_ref.clone();
        Callback::from(move |_| {
            let input = encode_shift_node_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                encode_shift_value_handle.set(input.value());
            }
        })
    };

    let decode_input_node_ref = use_node_ref();

    let decode_input_value_handle = use_state(String::default);
    let decode_input_value = (*decode_input_value_handle).clone();

    let decode_input_onchange = {
        let decode_input_node_ref = decode_input_node_ref.clone();
        Callback::from(move |_| {
            let input = decode_input_node_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                decode_input_value_handle.set(input.value());
            }
        })
    };

    let decode_shift_node_ref = use_node_ref();

    let decode_shift_value_handle = use_state(String::default);
    let decode_shift_value = (*decode_shift_value_handle).clone();

    let decode_shift_onchange = {
        let decode_shift_node_ref = decode_shift_node_ref.clone();
        Callback::from(move |_| {
            let input = decode_shift_node_ref.cast::<HtmlInputElement>();
            if let Some(input) = input {
                decode_shift_value_handle.set(input.value());
            }
        })
    };

    let encoded_text_handle = use_state(String::default);
    let encoded_text = (*encoded_text_handle).clone();
    let decoded_text_handle = use_state(String::default);
    let decoded_text = (*decoded_text_handle).clone();

    pub fn caesar_cipher_encrypt(text: &str, shift: i32) -> String {
        text.chars()
            .map(|c| match c {
                'a'..='z' => ((c as u8 - b'a' + shift as u8) % 26 + b'a') as char,
                'A'..='Z' => ((c as u8 - b'A' + shift as u8) % 26 + b'A') as char,
                _ => c,
            })
            .collect()
    }

    pub fn caesar_cipher_decrypt(text: &str, shift: i32) -> String {
        text.chars()
            .map(|c| match c {
                'a'..='z' => ((c as u8 - b'a' + 26 - shift as u8) % 26 + b'a') as char,
                'A'..='Z' => ((c as u8 - b'A' + 26 - shift as u8) % 26 + b'A') as char,
                _ => c,
            })
            .collect()
    }

    let on_encode_click = {
        let encoded_text_handle = encoded_text_handle.clone();
        let encode_input_value = encode_input_value.clone();
        let encode_shift_value = encode_shift_value.clone();
        Callback::from(move |_| {
            if let Ok(shift) = encode_shift_value.parse::<i32>() {
                encoded_text_handle.set(caesar_cipher_encrypt(&encode_input_value, shift));
            }
        })
    };

    let on_decode_click = {
        let decoded_text_handle = decoded_text_handle.clone();
        let decode_input_value = decode_input_value.clone();
        let decode_shift_value = decode_shift_value.clone();
        Callback::from(move |_| {
            if let Ok(shift) = decode_shift_value.parse::<i32>() {
                decoded_text_handle.set(caesar_cipher_decrypt(&decode_input_value, shift));
            }
        })
    };

    html! {
        <main class="min-h-screen flex justify-center items-center">
            <div>
                <h1 class="text-3xl font-bold">{" WebAssembly Implementation of Caesar Cipher Using Rust" }</h1>
                <h2 class="text-lg">{" Stephen Shkeda's Final Programming Languages Project "}</h2>

                <div class="mt-3">
                    <h1 class="text-xl font-semibold">
                        {" Encoder "}
                    </h1>
                    <div class="mt-2">
                        <input
                            ref={encode_input_node_ref}
                            type="text"
                            class="border p-1 outline-none rounded-md" placeholder="Enter text to encode..."
                            value={encode_input_value}
                            onchange={encode_input_onchange}
                        />

                        <input
                            ref={encode_shift_node_ref}
                            type="number"
                            class="border ml-2 p-1 outline-none rounded-md" placeholder="Shift amount"
                            value={encode_shift_value}
                            onchange={encode_shift_onchange}
                        />
                        <button
                            class="ml-2 border hover:bg-neutral-100 transition-colors duration-200 rounded-md py-1 px-2"
                            onclick={on_encode_click}
                        >{"Encode"}</button>
                    </div>
                    <h3 class="mt-2">{"Encoded Text:"}</h3>
                    <p>{encoded_text}</p>
                </div>

                <div class="mt-3">
                    <h1 class="text-xl font-semibold">
                        {" Decoder "}
                    </h1>
                    <div class="mt-2">
                        <input
                            type="text"
                            class="border p-1 outline-none rounded-md"
                            placeholder="Enter text to decode..."
                            ref={decode_input_node_ref}
                            value={decode_input_value}
                            onchange={decode_input_onchange}
                        />
                        <input
                            type="number"
                            class="border ml-2 p-1 outline-none rounded-md"
                            placeholder="Shift amount"
                            ref={decode_shift_node_ref}
                            value={decode_shift_value}
                            onchange={decode_shift_onchange}
                        />
                        <button
                            class="ml-2 border hover:bg-neutral-100 transition-colors duration-200 rounded-md py-1 px-2"
                            onclick={on_decode_click}
                        >{"Decode"}</button>
                    </div>
                    <h3 class="mt-2">{"Decoded Text:"}</h3>
                    <p>{decoded_text}</p>
                </div>

                <div class="mt-3">
                    <h1 class="text-xl">
                        {" Technologies Used "}
                    </h1>
                    <ul>
                        <li><p><span class="font-semibold">{"Rust"}</span>{": Used to implement the core logic for the Caesar cipher."}</p></li>
                        <li><p><span class="font-semibold">{"Yew"}</span>{": Used to develop the application using Rust."}</p></li>
                        <li><p><span class="font-semibold">{"WebAssembly (WASM)"}</span>{": The Rust code was compiled into WASM to run in the web browser."}</p></li>
                        <li><p><span class="font-semibold">{"Tailwind CSS"}</span>{": Used for styling the application."}</p></li>
                    </ul>
                </div>
            </div>
        </main>
    }
}
