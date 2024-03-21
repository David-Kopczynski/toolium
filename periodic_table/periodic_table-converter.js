const fs = require("fs")
const elements = require("./PubChemElements_all.json")

const parseF = (num) => parseFloat(num) == parseInt(num) ? `${parseInt(num)}.0` : num || ``
const parseSomeF = (num) => num ? `Some(${parseF(num)})` : 'None'

// Generate a list of elements and log
const transformed = (elements.Table.Row.map(({ Cell: element }) =>
    `Element {
        atomic_number: ${(element[0])},
        symbol: "${element[1]}",
        name: "${element[2]}",
        atomic_mass: ${parseF(element[3])},
        cpk_hex_color: ${element[4] ? `Some("${element[4]}")` : 'None'},
        electron_configuration: "${element[5]}",
        electronegativity: ${parseSomeF(element[6])},
        atomic_radius: ${parseSomeF(element[7])},
        ionization_energy: ${parseSomeF(element[8])},
        electron_affinity: ${parseSomeF(element[9])},
        oxidation_states: &[${element[10].length ? element[10].split(", ").map((num) => parseInt(num)).join(", ") : ""}],
        standard_state: "${element[11]}",
        melting_point: ${parseSomeF(element[12])},
        boiling_point: ${parseSomeF(element[13])},
        density: ${parseSomeF(element[14])},
        group_block: "${element[15]}",
        year_discovered: ${parseInt(element[16]) == element[16] ? `Some(${element[16]})` : "None"}
    }, `
).join("\n"))

// Write the list of elements to a file
fs.writeFileSync("elements.txt", transformed)
