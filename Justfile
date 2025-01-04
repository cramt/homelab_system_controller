dev_frontend:
    cd frontend && CARGO_BUILD_TARGET=wasm32-unknown-unknown dx serve

flash_pico:
    cd hardware_observer && CARGO_BUILD_TARGET=thumbv6m-none-eabi cargo run && sudo picotool reboot
