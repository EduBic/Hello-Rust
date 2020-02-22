
mod cnum;

fn main() {

    // let solver = cnum::RidderMethod {
    //     tol: 0.01
    // };

    // println!("{:?}", solver);

    // let fun : fn(f64) -> f64 = |x| x * x;

    // let sol = solver.solve(fun, 30.0, 20.0);
    // println!("{}", sol);
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ridder_method() {

        let test_fun = |x| x;

        let solver = super::cnum::RidderMethod {
            tol: 0.01,
            maxiter: 10,
            min_step: 1.0,
        };

        assert_eq!(solver.tol, 0.01);

        // Test base cases
        let solution = solver.solve(test_fun, -0.01, 10.0);
        assert_eq!(solution, -0.01);

        let solution_2 = solver.solve(test_fun, -10.0, 0.005);
        assert_eq!(solution_2, 0.005);

        // 
    }

}
