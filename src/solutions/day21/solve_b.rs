use crate::solutions::Solution;

// Did not finish, and I genuinely think this is the worst question this year.
// Obviously we won't perform the actual simulation, and the 'correct' approach
// seems pretty clear: run the simulation from different corners and sides of
// the field until you reach a steady state, then figure out how many copies
// of the field are in this steady state after the target number of steps,
// and figure out the number of active cells in the field copies that are
// not yet in the steady state. This is all doable – if a huge pain in the
// ass to implement – but it only works because the input has a couple of
// very specific properties: it has a border consisting of only open cells,
// and the center row and column are also open, meaning the 'wave-front'
// can always propagate in the cardinal directions unimpeded.
//
// If the input did not have these properties (like in the sample, actually),
// the question would have been even harder to solve, and I personally dis-
// like questions where you need to tailor the algorithm to a specific input;
// ideally, the solution should work on any input that matches the description,
// and conversely, if the solution that the author has in mind depends on pro-
// perties of the input, those properties should be mentioned in the text.
//
// Between my misgivings about the question and just how much of an error-
// prone slog it would be to implement, I decided to simply skip this one.

pub fn solve(_: &Vec<String>) -> Solution {
    return Solution::Integer(-1 as i64)
}
