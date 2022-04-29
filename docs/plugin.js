const sep = 8;

let memory_div = document.getElementById("memory");
let registers_div = document.getElementById("registers");
let hz_div = document.getElementById("hz");
let rom_keys = document.getElementById("rom_keys");
let rom_name = document.getElementById("rom_name");

for (let i=0; i < 4096; i+=sep) {
    memory_div.appendChild(document.createElement("div"));
}
for (let i=0; i < 16; i++) {
    registers_div.appendChild(document.createElement("div"));
}

miniquad_add_plugin({register_plugin: function (importObject) {
    importObject.env.draw_memory = function (js_object) {
        const memory = consume_js_object(js_object);
        for (let i=0; i < 4096; i+=sep) {
            let b = 0.1;
            for (let j=0; j < sep; j++) {
                b += memory[i + j]/255.0
            }
            memory_div.children[i/sep].style.backgroundColor = `rgba(255, 102, 201, ${b})`;
        }
    }

    importObject.env.draw_registers = function (js_object) {
        const registers = consume_js_object(js_object);
        for (let i=0; i < 16; i++) {
            registers_div.children[i].innerText = `${registers[i]}`;
        }
    }

}});

const ROM_KEYS = {
    "tetris": ["Q: rotate", "W: move left", "E: move right", "A: move down"],
    "brix": ["Q: move left", "E: move right"],
    "invaders": ["Q: move left", "W: shoot", "E: move right"],
    "pong": ["1: p1 move up", "Q: p1 move down", "4: p2 move up", "R: p2 move down"],
    "tictactoe": ["Each key represents a cell:", "1 2 3", "Q W E", "A S D"],
    "ibmlogo": ["No keys needed"],
    "particles": ["No keys needed"],
    "keypad": ["1 2 3 4", "Q W E R", "A S D F", "Z X C V"]
}

function update_keys(rom) {
    rom_name.innerText = `${rom} Instructions`;
    rom_keys.innerHTML = "";
    for (let ins of ROM_KEYS[rom]) {
        rom_keys.innerHTML += `<li>${ins}</li>`
    }
}
update_keys("brix");

function load_rom() {
    let rom_name = document.getElementById("rom").value;
    update_keys(rom_name);
    wasm_exports.load_rom(js_object(rom_name));
}

function update_hz(hz) {
    hz_div.innerText = `${hz}Hz`;
    wasm_exports.update_hz(js_object({"new_hz": hz}));
}

function toggle_bloom() {
    wasm_exports.toggle_bloom();
}



