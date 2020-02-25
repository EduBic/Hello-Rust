
#[derive(Debug)]
pub struct RidderMethod {
    pub tol: f64,
    pub maxiter: i32,
    pub min_step: f64
}

impl RidderMethod {
    
    pub fn solve(&self, fun: fn(f64) -> f64, x_low: f64, x_upp: f64) -> f64 {

        let f_low = fun(x_low);
        if f_low.abs() <= self.tol {
            return x_low;
        }

        let f_upp = fun(x_upp);
        if f_upp.abs() <= self.tol {
            return x_upp;
        }

        self.solve_iter(fun, x_low, f_low, x_upp, f_upp)
    }

    fn solve_iter(&self, fun: fn(f64) -> f64, 
                    mut x_low: f64, mut f_low: f64, 
                    mut x_upp: f64, mut f_upp: f64) -> f64 { 

        let mut xs = Vec::new();
        let mut fs = Vec::new();
        let mut i = 0;
        
        let mut error: f64 = 10E+10;
        let mut step: f64 = 10E+10;

        xs.push(x_upp);

        while error.abs() > self.tol && i < self.maxiter && step >= self.min_step {
            if RidderMethod::exist_x_axe_intersection(f_low, f_upp) {

                // compute x_new starting from x_mid
                let x_mid = (x_low + x_upp) / 2.0;
                let f_mid = fun(x_mid);

                let ridder_step = (f_mid.powi(2) - f_low * f_upp).sqrt();
                let x_tmp = (x_mid - x_low) * f_mid / ridder_step;

                // let x_new = x_mid + num_traits::sign::signum(f_low - f_upp) * x_tmp
                let x_new;
                if f_low >= f_upp {
                    x_new = x_mid + x_tmp;
                } else {
                    x_new = x_mid - x_tmp;
                }
                
                let f_new = fun(x_new);
                xs.push(x_new);
                fs.push(f_new);

                // update bounds
                // 
                // Example:
                // f_upp ---------------------------x--
                // 
                // f_new -----(2)--------(1)--x--(3)---
                //                            |
                // f_mid --------------x---------------
                //                     |      |
                //                     |      |
                //                     |      |
                // f_low -x----------------------------
                if RidderMethod::exist_x_axe_intersection(f_mid, f_new) { // (1)
                    x_low = x_mid;
                    f_low = f_mid;
                    x_upp = x_new; 
                    f_upp = f_new;
                } else if RidderMethod::exist_x_axe_intersection(f_low, f_new) {  // (2)
                    x_upp = x_new;
                    f_upp = f_new;
                } else if RidderMethod::exist_x_axe_intersection(f_upp, f_new) {  // (3)
                    x_low = x_new;
                    f_low = f_low;
                } else {
                    // TODO
                    panic!("Error: no intersection with x axe between bounds!");
                }

                error = f_new;
                step = (xs[(i + 1) as usize] - xs[i as usize]).abs();
                i += 1;
            } else {
                // doesn't exist x axe intersection
                panic!("Error: no intersection with x axe between bounds!");
            }
        }

        xs[i as usize]
    }

    fn exist_x_axe_intersection(f_low: f64, f_upp: f64)-> bool {
        f_low * f_upp <= 0.0
    }

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ridder_method_base_cases() {

        let test_fun = |x| x;

        let solver = super::RidderMethod {
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

        let solver = super::RidderMethod {
            tol: 0.01,
            maxiter: 30,
            min_step: 1.0,
        };

        // 
        let sol_3 = solver.solve(test_fun, -10.0, 10.0);
        assert_eq!(sol_3, -2.0);
    }

}

