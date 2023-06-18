use crate::*;

// pstr: ["t1: String", " t2: I32"]
pub fn parse_params(pstr: Vec<&str>) -> Vec<Param> {
    let mut params = vec![];

    for param_str in pstr {
        let tokens: Vec<&str> = param_str.split(':').map(|s| s.trim()).collect();
        let name = tokens[0];
        let ptype_str = tokens[1];
        let ptype = identify_type(ptype_str);

        params.push(Param {
            ptype,
            value: name.to_string(),
        })
    }

    return params;
}
