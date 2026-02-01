
fn main() {
	// Call the open_loop simulation with a results/images output directory
	lap_simulation::simulation::open_loop::open_loop("results/images", 0.1, 10.0);
}
