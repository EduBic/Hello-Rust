
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
    fn test_ridder_method_base_cases() {

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
    }

    #[test]
    fn test_ridder_method_iter_case() {
        let test_fun = |x| x + 2.0;

        let solver = super::cnum::RidderMethod {
            tol: 0.01,
            maxiter: 30,
            min_step: 1.0,
        };

        // 
        let sol_3 = solver.solve(test_fun, -10.0, 10.0);
        assert_eq!(sol_3, -2.0);
    }

}
