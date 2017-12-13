//Found by experimentation
const MAX_TRANSITIONS: u32 = 6;

const START: u32 = 0;
const END: u32 = 7;
const TARGET_NUMBER: i64 = 30;

type Transitions = Vec<Vec<(u32, usize)>>;

fn min9(x: i64) -> i64 {
    x - 9
}

fn min4(x: i64) -> i64 {
    x - 4
}

fn plus4(x: i64) -> i64 {
    x + 4
}

fn tim18(x: i64) -> i64 {
    x * 18
}

fn min18(x: i64) -> i64 {
    x - 18
}

fn min11(x: i64) -> i64 {
    x - 11
}

fn tim11(x: i64) -> i64 {
    x * 11
}

fn tim4(x: i64) -> i64 {
    x * 4
}

fn tim8(x: i64) -> i64 {
    x * 8
}

fn id(x: i64) -> i64 {
    x
}

fn tim9(x: i64) -> i64 {
    x * 9
}

fn min8(x: i64) -> i64 {
    x - 8
}

fn min1(x: i64) -> i64 {
    x - 1
}

static TRANS_TABLE: [fn(i64) -> i64; 13] = [
    min9,       //0
    plus4,      //1
    min4,       //2
    tim18,      //3
    min18,      //4
    tim11,      //5
    min11,      //6
    tim4,       //7
    tim8,       //8
    id,         //9
    tim9,       //10
    min8,       //11
    min1        //12
];
    

#[derive(Clone, Debug)]
struct State {
    current_node: u32,
    current_step: u32,
    expression_number: i64,
    path: Vec<u32>,
    trans: Vec<usize>,
}

impl State {
    fn new() -> State {
        State {
            current_node: START,
            current_step: 0,
            expression_number: 22,
            path: vec![],
            trans: vec![],
        }
    }
}

//TODO: Really clean this one up
fn main() {
    let transitions = build_transitions(); 
    let state = State::new();
    backtrack(state, &transitions);
}

//Stupid implementation because I gave up on closures and similar
fn build_transitions() -> Transitions {
    let mut res = vec![];
    let start = vec![
        (1, 0),
        (2, 1),
        (2, 2),
        (3, 1),
    ];
    res.push(start);
    let x1 = vec![
        (1, 0),
        (2, 2),
        (4, 3),
        (4, 4),
    ];
    res.push(x1);
    let x2 = vec![
        (1, 0),
        (2, 2),
        (2, 1),
        (2, 7),
        (4, 4),
        (5, 6),
        (5, 5),
        (6, 8),
        (3, 1),
        (3, 7),
    ];
    res.push(x2);
    let x3 = vec![
        (2, 1),
        (2, 7),
        (5, 5),
        (6, 8),
        (3, 7),
        (3, 1),
    ];
    res.push(x3);
    let x4 = vec![
        (4, 3),
        (4, 4),
        (1, 10),
        (2, 0),
        (5, 6),
        (5, 5),
        (2, 2),
        (END, 9),
    ];
    res.push(x4);
    let x5 = vec![
        (5, 5),
        (5, 6),
        (1, 0),
        (2, 2),
        (2, 7),
        (6, 8),
        (6, 11),
        (4, 3),
        (4, 4),
        (END, 9),
        (END, 12),
    ];
    res.push(x5);
    let x6 = vec![
        (6, 11),
        (6, 8),
        (3, 7),
        (5, 5),
        (5, 6),
        (END, 12),
    ];
    res.push(x6);
    let end = vec![];
    res.push(end);
    res
}

//If we need results somehow differently do a &mut in here
fn backtrack(state: State, transitions: &Transitions) {
    if reject(&state) {
        return;
    }
    if accept(&state) {
        println!("One solution: {:?}", state);
        return;
    }
    let pos_trans = &transitions[state.current_node as usize];
    //Try all the transitions one after another
    for trans in pos_trans {
        let mut new_state = state.clone();
        new_state.current_node = trans.0;
        new_state.current_step = state.current_step + 1;
        new_state.path.push(state.current_node);
        new_state.trans.push(trans.1);
        new_state.expression_number = TRANS_TABLE[trans.1](state.expression_number);
        backtrack(new_state, transitions);
    }
}

fn accept(state: &State) -> bool {
    state.current_node == END
    && state.expression_number == TARGET_NUMBER 
    && state.current_step <= MAX_TRANSITIONS
}

fn reject(state: &State) -> bool {
    //If our calculation gets negative we reject
    if state.expression_number < 0 {
        return true
    }
    //We have only one step left but we need at least two to make it to the end
    if state.current_step == MAX_TRANSITIONS - 1 
       && (state.current_node == 1 || state.current_node == 2 || state.current_node == 3) {
        return true
    }
    //We have no more steps left and are not on the end
    if state.current_step == MAX_TRANSITIONS && state.current_node != END {
        return true
    }
    false
}