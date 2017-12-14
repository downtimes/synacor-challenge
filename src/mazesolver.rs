use std::collections::HashMap;
use std::rc::Rc;

//NOTE: I tried it up to recursion depth 10 and it was still tolerable
//It finds 9648 solutions with reursion depth up to 10, so I hope a player would 
//come to one as short as 20 steps as well.
//Found by experimentation
const MAX_TRANSITIONS: u32 = 8;

//The Doors requested number
const TARGET_NUMBER: i64 = 30;
const START_NUMBER: i64 = 22;

//NOTE: We transformed our graph to a different graph
//Because every valid expression always ends on a number
//the numbers become our nodes and every transition is a combination
//of two steps in the grid. This tranformation was done by hand.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
enum Node {
    Start,
    Door,
    Left4,
    Top8,
    Bottom9,
    Right18,
    Middle11,
    Middle4,
}

#[derive(Debug)]
struct State {
    current_node: Node,
    current_step: u32,
    expression_number: i64,
    path: Vec<Node>,
    expression: String,
}

impl State {
    fn new() -> State {
        State {
            current_node: Node::Start,
            current_step: 0,
            expression_number: START_NUMBER,
            path: vec![],
            expression: String::new(),
        }
    }
}

//TODO: Really clean this one up
fn main() {
    let transitions = build_transitions();
    let state = State::new();
    let mut solutions = vec![];
    backtrack(state, &transitions, &mut solutions);
    println!("Number of solutions found: {}", solutions.len());
    println!(
        "The shortest solution is:\n{:?}",
        solutions.iter().min_by_key(|state| state.current_step)
    )
}

//Transitions tell us to wich new node we are going and contain the expression
//we need to evaluate to transition there
type RcType = Rc<(fn(i64) -> i64, String)>;
type Transition = (Node, RcType);
type Transitions = HashMap<Node, Vec<Transition>>;

//Not the most elegant approach to handcraft this thing ^^
fn build_transitions() -> Transitions {
    use Node::*;

    //All possible transition functions in the grid
    //The type inference is horrendous here so we have to help hard!
    //TODO: Search for bad error message 
    //https://play.rust-lang.org/?gist=8fc3d35987f091f3876ca8056e03a4b0&version=stable
    let min9: RcType = Rc::new((|x| x - 9, " - 9".to_owned()));
    let min4: RcType = Rc::new((|x| x - 4, " - 4".to_owned()));
    let plus4: RcType = Rc::new((|x| x + 4, " + 4".to_owned()));
    let min18: RcType = Rc::new((|x| x - 18, " - 18".to_owned()));
    let min1: RcType = Rc::new((|x| x - 1, " - 1".to_owned()));
    let min11: RcType = Rc::new((|x| x - 11, " - 11".to_owned()));
    let min8: RcType = Rc::new((|x| x - 8, " - 8".to_owned()));
    let tim4: RcType = Rc::new((|x| x * 4, " * 4".to_owned()));
    let tim18: RcType = Rc::new((|x| x * 18, " * 18".to_owned()));
    let tim11: RcType = Rc::new((|x| x * 11, " * 11".to_owned()));
    let tim8: RcType = Rc::new((|x| x * 8, " * 8".to_owned()));
    let tim9: RcType = Rc::new((|x| x * 9, " * 9".to_owned()));
    let id: RcType = Rc::new((|x| x, " * 1".to_owned()));

    let mut res = HashMap::new();
    //transitions from start
    let start = vec![
        (Bottom9, Rc::clone(&min9)),
        (Middle4, Rc::clone(&min4)),
        (Middle4, Rc::clone(&plus4)),
        (Left4, Rc::clone(&plus4)),
    ];
    res.insert(Start, start);
    //transitions from 9
    let bnine = vec![
        (Middle4, Rc::clone(&min4)),
        (Bottom9, Rc::clone(&min9)),
        (Bottom9, Rc::clone(&tim9)),
        (Right18, Rc::clone(&tim18)),
        (Right18, Rc::clone(&min18)),
        (Middle11, Rc::clone(&min11)),
    ];
    res.insert(Bottom9, bnine);
    //Transitions from middle 4
    let mfour = vec![
        (Middle4, Rc::clone(&min4)),
        (Middle4, Rc::clone(&plus4)),
        (Middle4, Rc::clone(&tim4)),
        (Bottom9, Rc::clone(&min9)),
        (Middle11, Rc::clone(&tim11)),
        (Middle11, Rc::clone(&min11)),
        (Left4, Rc::clone(&tim4)),
        (Left4, Rc::clone(&plus4)),
        (Right18, Rc::clone(&min18)),
        (Top8, Rc::clone(&tim8)),
    ];
    res.insert(Middle4, mfour);
    //Transitions from left 4
    let lfour = vec![
        (Left4, Rc::clone(&plus4)),
        (Left4, Rc::clone(&tim4)),
        (Top8, Rc::clone(&tim8)),
        (Middle11, Rc::clone(&tim11)),
        (Middle4, Rc::clone(&plus4)),
        (Middle4, Rc::clone(&tim4)),
    ];
    res.insert(Left4, lfour);
    //All the transitions from the right 18
    let reightteen = vec![
        (Right18, Rc::clone(&tim18)),
        (Right18, Rc::clone(&min18)),
        (Middle11, Rc::clone(&min11)),
        (Middle11, Rc::clone(&tim11)),
        (Bottom9, Rc::clone(&min9)),
        (Bottom9, Rc::clone(&tim9)),
        (Middle4, Rc::clone(&min4)),
        (Door, Rc::clone(&id)),
    ];
    res.insert(Right18, reightteen);
    //All transitions from the middle 11
    let meleven = vec![
        (Middle11, Rc::clone(&min11)),
        (Middle11, Rc::clone(&tim11)),
        (Right18, Rc::clone(&min18)),
        (Right18, Rc::clone(&tim18)),
        (Bottom9, Rc::clone(&min9)),
        (Middle4, Rc::clone(&min4)),
        (Middle4, Rc::clone(&tim4)),
        (Left4, Rc::clone(&tim4)),
        (Top8, Rc::clone(&tim8)),
        (Top8, Rc::clone(&min8)),
        (Door, Rc::clone(&id)),
        (Door, Rc::clone(&min1)),
    ];
    res.insert(Middle11, meleven);
    //All transitions from top 8
    let teight = vec![
        (Top8, Rc::clone(&min8)),
        (Top8, Rc::clone(&tim8)),
        (Left4, Rc::clone(&tim4)),
        (Middle4, Rc::clone(&tim4)),
        (Middle11, Rc::clone(&min11)),
        (Middle11, Rc::clone(&tim11)),
        (Door, Rc::clone(&min1)),
    ];
    res.insert(Top8, teight);
    res
}

fn backtrack(state: State, transitions: &Transitions, sol: &mut Vec<State>) {
    if reject(&state) {
        return
    }
    if accept(&state) {
        sol.push(state);
        return
    }
    if let Some(pos_trans) = transitions.get(&state.current_node) {
        //Try all the transitions one after another
        for trans in pos_trans {
            let new_state = create_next(&state, trans);
            backtrack(new_state, transitions, sol);
        }
    }
}

fn create_next(state: &State, trans: &Transition) -> State {
    let &(next_node, ref tran_rc) = trans;
    let mut new_path = state.path.clone();
    new_path.push(next_node);
    let mut new_exp = state.expression.clone();
    new_exp += tran_rc.1.as_str();
    State {
        current_node: next_node,
        current_step: state.current_step + 1,
        path: new_path,
        expression: new_exp,
        expression_number: tran_rc.0(state.expression_number),
    }
}

fn accept(state: &State) -> bool {
    state.current_node == Node::Door && state.expression_number == TARGET_NUMBER
        && state.current_step <= MAX_TRANSITIONS
}

fn reject(state: &State) -> bool {
    //If our calculation goes negative we reject
    if state.expression_number < 0 {
        return true;
    }
    //We have only one step left but we need at least two from these nodes
    //to make it to the end
    if state.current_step == MAX_TRANSITIONS - 1
        && (state.current_node == Node::Bottom9 
            || state.current_node == Node::Middle4 
            || state.current_node == Node::Left4)
    {
        return true;
    }
    //We have no more steps left and are not on the end
    if state.current_step == MAX_TRANSITIONS && state.current_node != Node::Door {
        return true;
    }

    //We don't know yet if we can reject this state
    false
}
