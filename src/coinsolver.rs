//Heap's algorithm from Wikipedia
fn generate_permutations(input: &[u32]) -> Vec<Vec<u32>> {
    let mut mem = vec![0; input.len()];
    let mut res = vec![];
    res.push(input.to_owned());

    let mut manipulated = input.to_owned();

    let mut i = 0;
    while i < input.len() {
        if mem[i] < i {
            if (i % 2) == 0 {
                manipulated.swap(0, i);
            } else {
                manipulated.swap(mem[i], i);
            }
            res.push(manipulated.clone());
            mem[i] += 1;
            i = 0;
        }  else {
            mem[i] = 0;
            i += 1;
        }
    }
    res
}

fn solves_equation(perm: &[u32]) -> bool {
    perm[0] + perm[1] * perm[2].pow(2) + perm[3].pow(3) - perm[4] == 399
}

fn number_to_coin(num: u32) -> String {
    match num {
        2 => "red coin".to_owned(),
        3 => "corroded coin".to_owned(),
        5 => "shiny coin".to_owned(),
        7 => "concave coin".to_owned(),
        9 => "blue coin".to_owned(),
        _ => "unknown coin".to_owned(),
    }
}

fn main() {
    //The coin values according to our game
    let input = [2, 3, 5, 7, 9];
    let perms = generate_permutations(&input);
    for perm in perms {
        if solves_equation(&perm) {
            println!("The right configuration is: {:?}", perm);
            println!("Corresponds to:\n\t{}\n\t{}\n\t{}\n\t{}\n\t{}",
                    number_to_coin(perm[0]),
                    number_to_coin(perm[1]),
                    number_to_coin(perm[2]),
                    number_to_coin(perm[3]),
                    number_to_coin(perm[4]));
        }
    }
}