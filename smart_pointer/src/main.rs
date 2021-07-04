mod box_;
mod drop_;
mod interior_mutability;
mod rc_;
mod reference_cycle;

fn main() {
    box_::main();
    drop_::main();
    rc_::main();
    interior_mutability::main();
    reference_cycle::main();
}
