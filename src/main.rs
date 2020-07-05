use std::env;

fn exit_with_usage() {
    println!("ERROR: invalid argument");
    println!("USAGE: ipconv <ipaddr>");
    std::process::exit(1);
}

fn parse_as_bits(i: u8) -> Vec<u8> {
    let mut bitvec = vec![];
    let mut i = i;

    for _ in 0..8 {
        bitvec.push(i & 1);
        i >>= 1;
    }

    bitvec.reverse();

    bitvec
}

fn main() {
    if let Some(ipaddr) = env::args().nth(1) {
        let block_vec = ipaddr.split('.').collect::<Vec<_>>();
        // assertion
        if block_vec.len() != 4 {
            exit_with_usage();
        }

        let mut results = vec![];

        for s in block_vec {
            if let Ok(block) = s.to_owned().parse::<u8>().as_mut() {
                let bitvec = parse_as_bits(*block);
                let bitstr = bitvec
                    .into_iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join("");

                results.push(bitstr);
            } else {
                exit_with_usage();
            }
        }

        let output = results.join(".");
        println!("{}", output);
    } else {
        exit_with_usage();
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_as_bits() {
        assert_eq!(parse_as_bits(0), vec![0, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(parse_as_bits(1), vec![0, 0, 0, 0, 0, 0, 0, 1]);
        assert_eq!(parse_as_bits(128), vec![1, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(parse_as_bits(254), vec![1, 1, 1, 1, 1, 1, 1, 0]);
        assert_eq!(parse_as_bits(255), vec![1, 1, 1, 1, 1, 1, 1, 1]);
    }
}
