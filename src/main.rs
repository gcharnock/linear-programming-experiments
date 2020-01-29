use lpsolve;

const IGNORED: f64 = 0.0;

fn main() {
    let mut problem = lpsolve::Problem::new(0, 2).unwrap();

    let objective = vec!(IGNORED, 0.0, -4.0);
    problem.set_objective_function(&objective);

    let constraint_1 = vec!(IGNORED, -1.0, 0.0);
    problem.add_constraint(&constraint_1, -1.0, lpsolve::ConstraintType::Ge);

    let constraint_2 = vec!(IGNORED, 4.0, -2.0);
    problem.add_constraint(&constraint_2, 0.0, lpsolve::ConstraintType::Ge);

    let constraint_3 = vec!(IGNORED, 0.0, 4.0);
    problem.add_constraint(&constraint_3, 0.0, lpsolve::ConstraintType::Ge);

    problem.write_lp(&mut std::io::stdout());
    println!();

    //problem.write_mps(&mut std::io::stdout(), 1);

    let solve_status = problem.solve();
    println!("solve_status = {:?}", solve_status);

    let mut solution = vec![0.0; 2];
    problem.get_solution_variables(&mut solution);
    println!("solution = {:?}", solution);
}


