# 🧑‍🔬 Periodic Table 
The periodic table has been sourced from https://pubchem.ncbi.nlm.nih.gov/periodic-table/ and is available in JSON format. The data is stored in the `PubChemElements_all.json` file. 
For the purpose of this project, the data has been transformed into a Rust struct. The struct is available in the `src/periodic_table.rs` file.

Transformation of the JSON data into a Rust struct was done using the JS script `periodic_table-converter.js` run with `node periodic_table-converter.js`. 
