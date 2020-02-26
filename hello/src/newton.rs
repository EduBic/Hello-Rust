
#[derive(Debug)]
pub struct NewtonMethod {
    pub tol_f: f64,
    pub max_iter: i32,

    pub abs_step: f64,
    pub rel_step: f64,

    pub min_x_step: f64,
    pub min_f_x_step: f64
}


impl NewtonMethod {
    
    pub fn solve(self, x0 : f64, fun : (f64) -> f64) -> (f64, f64) {
    
        
        // TODO: check if x0 is in admissible area
        let f_x0 = fun(x0);
        let f_error = f_x0; // L2 norm

        if f_error < self.tol_f {
            // end convergence
            (x0, f_x0)
        }

        // iterative case
        // TODO: consider min_step
        let mut curr_x = x0;
        let mut curr_fx = f_x0;
        let mut iter = 0;
        while f_error > self.tol_f && iter < self.max_iter {
            
            let h = abs_step;
            let fun_derivative_val = (fun(curr_x + h) - fun(curr_x)) / h;
            let deltaX = - fun(curr_x) / fun_derivative_val;
            
            curr_x = curr_x + deltaX;
            curr_fx = fun(curr_fx);
            f_error = curr_fx;
            
            iter += 1;
        }

        (curr_x, curr_fx)
    }

    fn compute_h(self) -> f64 {

    }
}